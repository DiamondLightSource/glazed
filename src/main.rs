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
async fn main() {
    let Ok(config) = GlazedConfig::from_file(&"config.toml") else {
        eprintln!("Failed to load config");
        exit(1);
    };

    let schema = Schema::build(TiledQuery(TiledClient), EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler::<TiledClient>))
        .layer(Extension(schema));

    let Ok(listener) = tokio::net::TcpListener::bind(config.bind_address).await else {
        eprintln!("Failed to bind TCP Listener");
        exit(1);
    };

    let Ok(_) = axum::serve(listener, app).await else {
        eprintln!("Failed to serve");
        exit(1);
    };
}
