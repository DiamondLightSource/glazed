use async_graphql::{Context, SimpleObject, Subscription, Union};
use axum::http::HeaderMap;
use futures_util::{Stream, StreamExt};
use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tracing::{error, info};

use crate::clients::TiledClient;
use crate::handlers::AuthHeader;
use crate::model::array::{ArrayStructure, DataType};

pub struct TiledSubscription;

#[Subscription]
impl TiledSubscription {
    async fn events(&self, ctx: &Context<'_>) -> impl Stream<Item = TiledEvent> {
        let client = ctx
            .data::<TiledClient>()
            .expect("TiledClient not found in context");
        let auth = ctx
            .data_opt::<Option<AuthHeader>>()
            .and_then(|a| a.as_ref());
        let headers = auth.map(|a| a.as_header_map());

        stream_events(client, None, headers)
    }

    async fn node_events(&self, ctx: &Context<'_>, node: String) -> impl Stream<Item = TiledEvent> {
        let client = ctx
            .data::<TiledClient>()
            .expect("TiledClient not found in context");
        let auth = ctx
            .data_opt::<Option<AuthHeader>>()
            .and_then(|a| a.as_ref());
        let headers = auth.map(|a| a.as_header_map());
        stream_events(client, Some(node), headers)
    }
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct ContainerSchema {
    pub version: u32,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct ArraySchema {
    pub version: u32,
    pub data_type: DataType,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct TableSchema {
    pub version: u32,
    pub arrow_schema: String,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct ChildCreated {
    pub sequence: u32,
    pub timestamp: String,
    pub key: String,
    pub structure_family: String,
    pub specs: Value,
    pub metadata: Value,
    pub data_sources: Vec<DataSource>,
    pub access_blob: Value,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct ChildMetadataUpdated {
    pub sequence: u32,
    pub timestamp: String,
    pub key: String,
    pub specs: Value,
    pub metadata: Value,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct StreamArrayData {
    pub sequence: u32,
    pub timestamp: String,
    pub mimetype: String,
    pub shape: Value,
    pub offset: Option<Value>,
    pub block: Option<Value>,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct ArrayRef {
    pub sequence: u32,
    pub timestamp: String,
    pub data_source: Value,
    pub patch: Option<Value>,
    pub uri: Option<Value>,
    pub shape: Value,
    pub data_type: DataType,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct StreamTableData {
    pub sequence: u32,
    pub timestamp: String,
    pub partition: Option<i32>,
    pub append: bool,
    pub arrow_schema: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, SimpleObject)]
pub struct DataSource {
    pub structure: ArrayStructure,
}

#[derive(Union, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TiledEvent {
    #[serde(rename = "container-schema")]
    Container(ContainerSchema),
    #[serde(rename = "array-schema")]
    Array(ArraySchema),
    #[serde(rename = "table-schema")]
    Table(TableSchema),
    #[serde(rename = "container-child-created")]
    ChildCreated(ChildCreated),
    #[serde(rename = "container-child-metadata-updated")]
    ChildMetadataUpdated(ChildMetadataUpdated),
    #[serde(rename = "array-data")]
    ArrayData(StreamArrayData),
    #[serde(rename = "array-ref")]
    ArrayRef(ArrayRef),
    #[serde(rename = "table-data")]
    TableData(StreamTableData),
}

fn stream_events(
    client: &TiledClient,
    node: Option<String>,
    headers: Option<HeaderMap>,
) -> impl Stream<Item = TiledEvent> {
    let mut url = client.address().clone();

    let scheme = match url.scheme() {
        "http" => "ws",
        "https" => "wss",
        _ => "ws",
    };
    url.set_scheme(scheme).ok();

    let path = if let Some(node_id) = node {
        format!("api/v1/stream/single/{}", node_id)
    } else {
        "api/v1/stream/single/".to_string()
    };

    let mut url = url.join(&path).expect("Invalid stream path");
    url.set_query(Some("envelope_format=msgpack"));

    let mut request = url.as_str().into_client_request().unwrap();
    if let Some(headers) = headers {
        request.headers_mut().extend(headers);
    }
    async_stream::stream! {
        info!("Connecting to WebSocket: {}", url);
        let (ws_stream, _) = match connect_async(request).await {
            Ok(ws) => ws,
            Err(e) => {
                error!("Failed to connect to WebSocket: {}", e);
                return;
            }
        };

        let (_, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            match msg {
                Ok(tokio_tungstenite::tungstenite::Message::Binary(bin)) => {
                    match rmp_serde::from_slice::<TiledEvent>(&bin) {
                        Ok(event) => yield event,
                        Err(e) => {
                            error!("Failed to deserialize msgpack: {}, binary: {:?}", e, bin);
                        }
                    }
                }
                Ok(_) => {},
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderMap;
    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_hdr_async;
    use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
    use url::Url;

    use super::*;
    use crate::clients::TiledClient;

    #[tokio::test]
    async fn test_stream_events_headers() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = Url::parse(&format!("http://{addr}")).unwrap();
        let client = TiledClient::new(url);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer test-token".parse().unwrap());

        let headers_clone = headers.clone();

        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut captured_headers = HeaderMap::new();

            let callback = |req: &Request, response: Response| {
                for (name, value) in req.headers() {
                    captured_headers.insert(name.clone(), value.clone());
                }
                Ok(response)
            };

            let _ws_stream = accept_hdr_async(stream, callback).await.unwrap();
            captured_headers
        });

        let stream = stream_events(&client, None, Some(headers_clone));
        tokio::pin!(stream);

        let _ = stream.next().await;

        let captured_headers = server_handle.await.unwrap();

        assert_eq!(
            captured_headers.get("Authorization").unwrap(),
            "Bearer test-token"
        );
    }

    #[tokio::test]
    async fn test_subscription_events_and_node_events() {
        use async_graphql::{EmptyMutation, Schema};
        use tokio_tungstenite::tungstenite::Message;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = Url::parse(&format!("http://{addr}")).unwrap();
        let client = TiledClient::new(url);

        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut captured_path = String::new();
            let callback = |req: &Request, response: Response| {
                captured_path = req.uri().path().to_string();
                Ok(response)
            };
            let mut ws_stream = accept_hdr_async(stream, callback).await.unwrap();

            let event = serde_json::json!({
                "type": "container-schema",
                "version": 42
            });
            let bin = rmp_serde::to_vec(&event).unwrap();
            ws_stream.send(Message::Binary(bin.into())).await.unwrap();

            let (stream, _) = listener.accept().await.unwrap();
            let mut captured_node_path = String::new();
            let callback = |req: &Request, response: Response| {
                captured_node_path = req.uri().path().to_string();
                Ok(response)
            };
            let mut ws_stream = accept_hdr_async(stream, callback).await.unwrap();

            let event = serde_json::json!({
                "type": "container-schema",
                "version": 43
            });
            let bin = rmp_serde::to_vec(&event).unwrap();
            ws_stream.send(Message::Binary(bin.into())).await.unwrap();

            (captured_path, captured_node_path)
        });

        let schema = Schema::build(crate::model::TiledQuery, EmptyMutation, TiledSubscription)
            .data(client)
            .finish();

        let stream =
            schema.execute_stream("subscription { events { ... on ContainerSchema { version } } }");
        let mut stream = Box::pin(stream);
        let res = stream.next().await.unwrap();
        assert_eq!(
            res.data,
            async_graphql::value!({ "events": { "version": 42 } })
        );

        let stream = schema.execute_stream(
            "subscription { nodeEvents(node: \"foo\") { ... on ContainerSchema { version } } }",
        );
        let mut stream = Box::pin(stream);
        let res = stream.next().await.unwrap();
        assert_eq!(
            res.data,
            async_graphql::value!({ "nodeEvents": { "version": 43 } })
        );

        let (path, node_path) = server_handle.await.unwrap();
        assert_eq!(path, "/api/v1/stream/single/");
        assert_eq!(node_path, "/api/v1/stream/single/foo");
    }
}
