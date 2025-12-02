use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use axum::body::Body;
use axum::extract::{OptionalFromRequestParts, Path, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse};
use reqwest::header::AUTHORIZATION;
use tracing::info;

use crate::clients::TiledClient;
use crate::model::TiledQuery;

pub async fn graphql_handler(
    auth_token: Option<AuthHeader>,
    schema: Extension<Schema<TiledQuery, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(auth_token))
        .await
        .into()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn download_handler(
    auth: Option<AuthHeader>,
    State(client): State<TiledClient>,
    Path((run, stream, det, id)): Path<(String, String, String, u32)>,
) -> (StatusCode, HeaderMap, Body) {
    info!("Downloading {run}/{stream}/{det}/{id}");
    let headers = auth.as_ref().map(AuthHeader::as_header_map);
    let req = client.download(run, stream, det, id, headers).await;
    crate::download::forward_download_response(req).await
}

/// Extractor to accept an un-typed Authorization header (can be ApiKey/Bearer/Basic etc), and
/// make it accessible as a HeaderValue to be forwarded rather than extracted into something to use
/// locally (as the TypedHeader equivalent does).
pub struct AuthHeader(HeaderValue);

impl AuthHeader {
    pub fn as_header_map(&self) -> HeaderMap {
        [(AUTHORIZATION, self.0.clone())].into_iter().collect()
    }
}

impl<S> OptionalFromRequestParts<S> for AuthHeader
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(parts
            .headers
            .get("Authorization")
            .map(|value| Self(value.clone())))
    }
}
