use std::error;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::{get, post};
use axum::{Extension, Router};

mod cli;
mod clients;
mod config;
mod handlers;
mod model;

use cli::{Cli, Commands};

use crate::clients::TiledClient;
use crate::config::GlazedConfig;
use crate::handlers::{graphiql_handler, graphql_handler};
use crate::model::TiledQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::init();

    let config_filepath = cli.config_filepath.unwrap_or("config.toml".into());

    let config = GlazedConfig::from_file(&config_filepath)?;

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

    axum::serve(listener, app).await?;

    Ok(())
}
