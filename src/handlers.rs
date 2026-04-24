use async_graphql::http::{ALL_WEBSOCKET_PROTOCOLS, GraphiQLSource};
use async_graphql::{EmptyMutation, Schema};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::Extension;
use axum::body::Body;
use axum::extract::{OptionalFromRequestParts, Path, State, WebSocketUpgrade};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse};
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
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
    auth_token: Option<AuthHeader>,
    websocket: WebSocketUpgrade,
) -> impl IntoResponse {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |socket| {
            GraphQLWebSocket::new(socket, schema.0, protocol)
                .on_connection_init(|value| async move {
                    let mut data = async_graphql::Data::default();
                    let mut auth_token = auth_token;

                    if let Ok(value) = serde_json::from_value::<Value>(value)
                        && let Some(auth) = value.get("Authorization").and_then(|v| v.as_str())
                        && let Ok(header_value) = HeaderValue::from_str(auth)
                    {
                        auth_token = Some(AuthHeader(header_value));
                    }

                    data.insert(auth_token);
                    Ok(data)
                })
                .serve()
        })
}

pub async fn graphiql_handler(graphql_endpoint: Option<String>) -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(graphql_endpoint.as_deref().unwrap_or("/graphql"))
            .subscription_endpoint(graphql_endpoint.as_deref().unwrap_or("/ws"))
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
    use crate::clients::TiledClient;

    async fn auth_echo(auth: Option<AuthHeader>) -> impl IntoResponse {
        match auth {
            Some(header) => header.0.to_str().unwrap().to_owned(),
            None => "No auth".to_owned(),
        }
    }
    fn app() -> Router {
        Router::new().route("/", get(auth_echo))
    }
    use async_graphql::{EmptyMutation, Schema};
    use axum::Extension;
    use axum::routing::any;

    fn ws_app(client: TiledClient) -> Router {
        let schema = Schema::build(
            crate::model::TiledQuery,
            EmptyMutation,
            crate::model::subscription::TiledSubscription,
        )
        .data(client)
        .finish();
        Router::new().route(
            "/ws",
            any(
                move |protocol: async_graphql_axum::GraphQLProtocol,
                      websocket: axum::extract::WebSocketUpgrade| {
                    let schema = Extension(schema.clone());
                    super::graphql_ws_handler(schema, protocol, None, websocket)
                },
            ),
        )
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

    #[tokio::test]
    async fn test_graphql_ws_handler_auth_extraction() {
        use futures_util::{SinkExt, StreamExt};
        use serde_json::json;
        use tokio_tungstenite::connect_async;
        use tokio_tungstenite::tungstenite::Message;

        use crate::clients::TiledClient;

        let server = httpmock::MockServer::start();
        let client = TiledClient::for_mock_server(&server);

        let app = ws_app(client);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let ws_url = format!("ws://{addr}/ws");
        let mut request =
            tokio_tungstenite::tungstenite::client::IntoClientRequest::into_client_request(ws_url)
                .unwrap();
        request.headers_mut().insert(
            axum::http::header::SEC_WEBSOCKET_PROTOCOL,
            "graphql-transport-ws".parse().unwrap(),
        );
        let (mut ws_stream, _) = connect_async(request).await.unwrap();

        let init_msg = json!({
            "type": "connection_init",
            "payload": {
                "Authorization": "Bearer ws-test-token"
            }
        });
        ws_stream
            .send(Message::Text(init_msg.to_string().into()))
            .await
            .unwrap();

        let ack = ws_stream.next().await.unwrap().unwrap();
        assert!(ack.is_text());
        assert!(ack.to_text().unwrap().contains("connection_ack"));

        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path("/api/v1/metadata/5d8f5c3e-0e00-4c5c-816d-70b4b0f41498")
                    .header("Authorization", "Bearer ws-test-token");
                then.status(200)
                    .body_from_file("resources/metadata_run.json");
            })
            .await;

        let query = json!({
            "id": "1",
            "type": "subscribe",
            "payload": {
                "query": "query { run(id: \"5d8f5c3e-0e00-4c5c-816d-70b4b0f41498\") { id } }"
            }
        });
        ws_stream
            .send(Message::Text(query.to_string().into()))
            .await
            .unwrap();

        let msg = ws_stream.next().await.unwrap().unwrap();
        assert!(msg.is_text());
        assert!(
            msg.to_text()
                .unwrap()
                .contains("5d8f5c3e-0e00-4c5c-816d-70b4b0f41498")
        );

        mock.assert_async().await;
    }
}
