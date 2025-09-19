use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::{
    clients::tiled_client::TiledClient,
    schemas::tiled::TiledSchema,
};
 
pub async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
    let schema = Schema::build(
        TiledSchema(TiledClient),
        EmptyMutation,
        EmptySubscription,
    )
    .finish();

    let query = req.into_inner().query;

    schema.execute(query).await.into()
}