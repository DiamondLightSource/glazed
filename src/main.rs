use std::error;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::post;
use axum::{Extension, Router};

mod cli;
mod clients;
mod config;
mod handlers;
mod schemas;

use cli::{Cli, Commands};

use crate::clients::tiled_client::TiledClient;
use crate::config::GlazedConfig;
use crate::handlers::graphql::graphql_handler;
use crate::schemas::TiledQuery;

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
        .route("/graphql", post(graphql_handler::<TiledClient>))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
