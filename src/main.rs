use async_graphql::{EmptyMutation, Schema};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};

mod cli;
mod clients;
mod config;
mod download;
mod handlers;
mod model;
#[cfg(test)]
mod test_utils;

use cli::{Cli, Commands};
use serde_json::json;
use tokio::select;
use tokio::signal::unix::{SignalKind, signal};
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

use crate::clients::TiledClient;
use crate::config::GlazedConfig;
use crate::handlers::{download_handler, graphiql_handler, graphql_handler, graphql_ws_handler};
use crate::model::TiledQuery;
use crate::model::subscription::TiledSubscription;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::init();
    let config;

    if let Some(config_filepath) = cli.config_filepath {
        info!("Loading config from {config_filepath:?}");
        config = GlazedConfig::from_file(&config_filepath)?;
        info!("Config loaded");
    } else {
        info!("Using default config");
        config = GlazedConfig::default();
    }
    tracing_subscriber::registry()
        .with(LevelFilter::from(config.log_level))
        .with(fmt::Layer::default())
        .init();

    match cli.command {
        Commands::Serve => serve(config).await,
    }
}

#[derive(Clone)]
pub struct RootAddress(Url);

async fn serve(config: GlazedConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = TiledClient::new(config.tiled_client.address.clone());
    let schema = Schema::build(TiledQuery, EmptyMutation, TiledSubscription)
        .data(RootAddress(config.public_address.clone()))
        .data(client.clone())
        .finish();

    let graphql_endpoint = config.endpoint("graphql");
    let graphiql_endpoint = config.endpoint("graphiql");
    let subscription_endpoint = config.endpoint("subscribe");
    info!(
        "Redirecting traffic to public address: {}",
        config.public_address
    );
    info!("Public graphql endpoint available at {}", graphql_endpoint);

    let page_not_found = (
        StatusCode::NOT_FOUND,
        not_found_page(&graphql_endpoint, &graphiql_endpoint),
    );

    let app = Router::new()
        .route("/graphql", post(graphql_handler).get(graphql_get_warning))
        .route("/subscribe", get(graphql_ws_handler))
        .route(
            "/graphiql",
            get(|| graphiql_handler(graphql_endpoint, subscription_endpoint)),
        )
        .route(
            "/status",
            get(Json(json!({"version": env!("CARGO_PKG_VERSION")}))),
        )
        .route("/asset/{run}/{stream}/{det}/{id}", get(download_handler))
        .with_state(client)
        .fallback(page_not_found)
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("Serving glazed at {:?}", config.bind_address);

    Ok(axum::serve(listener, app)
        .with_graceful_shutdown(signal_handler())
        .await?)
}

fn not_found_page(graphql: &str, graphiql: &str) -> Html<String> {
    Html(format!(
        include_str!("../templates/404.html"),
        graphql_address = graphql,
        graphiql_address = graphiql
    ))
}

async fn graphql_get_warning() -> impl IntoResponse {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        [("Allow", "POST")],
        Html(include_str!("../static/get_graphql_warning.html")),
    )
}

async fn signal_handler() {
    let mut term = signal(SignalKind::terminate()).expect("Failed to create SIGTERM listener");
    let mut int = signal(SignalKind::interrupt()).expect("Failed to create SIGINT listener");
    let mut quit = signal(SignalKind::quit()).expect("Failed to create SIGQUIT listener");
    let sig = select! {
         _ = term.recv() => "SIGTERM",
        _ = int.recv() => "SIGINT",
        _ = quit.recv() => "SIGQUIT",
    };
    info!("Server interrupted by {sig}");
}

#[cfg(test)]
mod tests {
    use super::not_found_page;

    #[test]
    fn test_404() {
        let response = not_found_page(
            "http://example.com/glazed/graphql",
            "http://example.com/glazed/graphiql",
        );

        assert_eq!(
            response.0,
            r#"<!doctype html>
<html>
    <head>
        <title>Glazed</title>
    </head>
    <body>
        <h1>GraphQL interface to Tiled</h1>
        <p>
            Service is available at
            <a href="http://example.com/glazed/graphql">/graphql</a>.
            Playground is available for testing at
            <a href="http://example.com/glazed/graphiql">/graphiql</a>
        </p>
    </body>
</html>
"#
        )
    }
}
