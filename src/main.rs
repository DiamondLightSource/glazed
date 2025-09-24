use std::process::exit;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::post;
use axum::{Extension, Router};

mod clients;
mod config;
mod handlers;
mod schemas;

use crate::clients::tiled_client::TiledClient;
use crate::handlers::graphql::graphql_handler;
use crate::schemas::TiledQuery;

#[tokio::main]
async fn main() {
    let schema = Schema::build(TiledQuery(TiledClient), EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler::<TiledClient>))
        .layer(Extension(schema));

    let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3000").await else {
        eprintln!("Failed to bind TCP Listener");
        exit(1);
    };

    let Ok(_) = axum::serve(listener, app).await else {
        eprintln!("Failed to serve");
        exit(1);
    };
}
