use async_graphql::*;
use axum::{
    routing::post, Router
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use crate::schemas::tiled::TiledSchema;

mod schemas;
mod clients;

async fn graphql_handler(req: GraphQLRequest)-> GraphQLResponse{

    let tiled_client = clients::mock_tiled_client::MockTiledClient;

    let schema = Schema::build(TiledSchema, EmptyMutation, EmptySubscription)
        .data(tiled_client)
        .finish();

    let query = req.into_inner().query;

    schema.execute(query).await.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/graphql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

}
