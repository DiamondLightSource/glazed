use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use axum::response::{Html, IntoResponse};

use crate::clients::Client;
use crate::model::TiledQuery;

pub async fn graphql_handler<T: Client + Send + Sync + 'static>(
    schema: Extension<Schema<TiledQuery, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let query = req.into_inner().query;

    schema.execute(query).await.into()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
