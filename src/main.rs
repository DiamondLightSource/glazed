use std::error;
use std::process::exit;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::post;
use axum::{Extension, Router};

mod clients;
mod config;
mod handlers;
mod schemas;

use crate::clients::tiled_client::TiledClient;
use crate::config::GlazedConfig;
use crate::handlers::graphql::graphql_handler;
use crate::schemas::TiledQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let config = GlazedConfig::from_file("config.toml")?;

    let schema = Schema::build(
        TiledQuery(TiledClient {
            address: config.tiled_client.address.to_owned(),
        }),
        EmptyMutation,
        EmptySubscription
    )
    .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler::<TiledClient>))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;

    axum::serve(listener, app).await?;

    exit(0);
}
