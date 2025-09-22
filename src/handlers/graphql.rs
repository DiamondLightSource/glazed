use crate::{clients::client::Client, schemas::tiled::TiledSchema};
use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

pub async fn graphql_handler<T: Client + Send + Sync + 'static>(req: GraphQLRequest) -> GraphQLResponse {
    let client = T::new();
    let schema = Schema::build(TiledSchema(client), EmptyMutation, EmptySubscription).finish();

    let query = req.into_inner().query;

    schema.execute(query).await.into()
}
