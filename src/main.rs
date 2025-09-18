use axum::{Router, routing::post};

mod clients;
mod handlers;
mod schemas;

use crate::{
    handlers::graphql::graphql_handler,
};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/graphql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
