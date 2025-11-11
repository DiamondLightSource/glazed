use std::error;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
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
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("Serving glazed at {:?}", config.bind_address);

    Ok(axum::serve(listener, app).await?)
}
