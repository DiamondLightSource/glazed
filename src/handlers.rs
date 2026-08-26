use async_graphql::http::{ALL_WEBSOCKET_PROTOCOLS, GraphiQLSource};
use async_graphql::{Data, EmptyMutation, Schema};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::Extension;
use axum::body::Body;
use axum::extract::{OptionalFromRequestParts, Path, State, WebSocketUpgrade};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse};
use reqwest::header::AUTHORIZATION;
use tracing::info;

use crate::clients::TiledClient;
use crate::model::TiledQuery;
use crate::model::subscription::TiledSubscription;

pub async fn graphql_handler(
    auth_token: Option<AuthHeader>,
    schema: Extension<Schema<TiledQuery, EmptyMutation, TiledSubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(auth_token))
        .await
        .into()
}

pub async fn graphql_ws_handler(
    schema: Extension<Schema<TiledQuery, EmptyMutation, TiledSubscription>>,
    protocol: GraphQLProtocol,
    mut auth_token: Option<AuthHeader>,
    websocket: WebSocketUpgrade,
) -> impl IntoResponse {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |socket| {
            GraphQLWebSocket::new(socket, schema.0, protocol)
                .on_connection_init(|headers| async move {
                    let mut data = Data::default();

                    auth_token = headers
                        .get("Authorization")
                        .and_then(|v| v.as_str())
                        .and_then(|auth| HeaderValue::from_str(auth).ok())
                        .map(AuthHeader)
                        .or(auth_token);

                    data.insert(auth_token);
                    Ok(data)
                })
                .serve()
        })
}

pub async fn graphiql_handler(
    graphql_endpoint: String,
    subscription_endpoint: String,
) -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(&graphql_endpoint)
            .subscription_endpoint(&subscription_endpoint)
            .finish(),
    )
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

#[cfg(test)]
impl From<HeaderValue> for AuthHeader {
    fn from(value: HeaderValue) -> Self {
        Self(value)
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

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::response::IntoResponse;
    use axum::routing::get;
    use http_body_util::BodyExt as _;
    use tower::ServiceExt;

    use super::AuthHeader;

    async fn auth_echo(auth: Option<AuthHeader>) -> impl IntoResponse {
        match auth {
            Some(header) => header.0.to_str().unwrap().to_owned(),
            None => "No auth".to_owned(),
        }
    }
    fn app() -> Router {
        Router::new().route("/", get(auth_echo))
    }

    #[tokio::test]
    async fn auth_extract() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Authorization", "auth_value")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response.into_body().collect().await.unwrap().to_bytes(),
            "auth_value"
        );
    }
    #[tokio::test]
    async fn no_auth_extract() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(
            response.into_body().collect().await.unwrap().to_bytes(),
            "No auth"
        );
    }
}
