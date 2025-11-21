use std::error;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Extension, Router};

mod cli;
mod clients;
mod config;
mod handlers;
mod model;
#[cfg(test)]
mod test_utils;

use cli::{Cli, Commands};
use tokio::select;
use tokio::signal::unix::{SignalKind, signal};
use tracing::info;

use crate::clients::TiledClient;
use crate::config::GlazedConfig;
use crate::handlers::{graphiql_handler, graphql_handler};
use crate::model::TiledQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

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
    match cli.command {
        Commands::Serve => serve(config).await,
    }
}

async fn serve(config: GlazedConfig) -> Result<(), Box<dyn error::Error>> {
    let schema = Schema::build(
        TiledQuery(TiledClient {
            address: config.tiled_client.address.to_owned(),
        }),
        EmptyMutation,
        EmptySubscription,
    )
    .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql_handler))
        .route("/asset", get(download_file))
        .fallback((
            StatusCode::NOT_FOUND,
            Html(include_str!("../static/404.html")),
        ))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("Serving glazed at {:?}", config.bind_address);

    Ok(axum::serve(listener, app)
        .with_graceful_shutdown(signal_handler())
        .await?)
}

async fn download_file() -> impl IntoResponse {
    let client = reqwest::Client::new();
    let req = client.get("http://localhost:8407/api/v1/asset/bytes/77e40dcd-00f7-48aa-9eeb-1909c3ce5831/primary/det?id=1").send().await.unwrap();
    Body::from_stream(req.bytes_stream())
}

async fn signal_handler() {
    let mut term = signal(SignalKind::terminate()).expect("Failed to create SIGTERM listener");
    let mut int = signal(SignalKind::interrupt()).expect("Failed to create SIGINT listener");
    let mut quit = signal(SignalKind::interrupt()).expect("Failed to create SIGQUIT listener");
    let sig = select! {
         _ = term.recv() => "SIGTERM",
        _ = int.recv() => "SIGINT",
        _ = quit.recv() => "SIGQUIT",
    };
    info!("Server interrupted by {sig}");
}
