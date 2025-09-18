use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, routing::post};

mod clients;
mod schemas;

use crate::{
    clients::mock_tiled_client::MockTiledClient, 
    clients::tiled_client::TiledClient,
    schemas::{metadata::Metadata, tiled::TiledSchema}
};

trait Client {
    fn get_metadata_struct(&self) -> impl Future<Output = Metadata> + Send;
}

async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
    let schema = Schema::build(
        TiledSchema(MockTiledClient),
        EmptyMutation,
        EmptySubscription,
    )
    .finish();

    let query = req.into_inner().query;

    schema.execute(query).await.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/graphql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
