use axum::{Router, routing::post};
use std::process::exit;

mod clients;
mod handlers;
mod schemas;

use crate::{clients::mock_tiled_client::MockTiledClient, handlers::graphql::graphql_handler};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/graphql", post(graphql_handler::<MockTiledClient>));

    let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3000").await else {
        eprintln!("Failed to bind TCP Listener");
        exit(1);
        };

    let Ok(_) = axum::serve(listener, app).await else {
        eprintln!("Failed to serve");
        exit(1);
        };
}
