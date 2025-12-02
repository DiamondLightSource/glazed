use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse};
use tracing::info;

use crate::clients::TiledClient;
use crate::model::TiledQuery;

pub async fn graphql_handler(
    schema: Extension<Schema<TiledQuery, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let query = req.into_inner().query;
    schema.execute(query).await.into()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn download_handler(
    State(client): State<TiledClient>,
    Path((run, stream, det, id)): Path<(String, String, String, u32)>,
) -> (StatusCode, HeaderMap, Body) {
    info!("Downloading {run}/{stream}/{det}/{id}");
    let req = client.download(run, stream, det, id).await;
    crate::download::forward_download_response(req).await
}
