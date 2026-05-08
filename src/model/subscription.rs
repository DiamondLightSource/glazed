use async_graphql::{Context, SimpleObject, Subscription, Union};
use futures_util::Stream;
use serde::Deserialize;
use serde_json::Value;

use crate::clients::TiledClient;
use crate::handlers::AuthHeader;
use crate::model::array::{ArrayStructure, DataType};

pub struct TiledSubscription;

#[Subscription]
impl TiledSubscription {
    async fn events(&self, ctx: &Context<'_>) -> impl Stream<Item = TiledEvent> {
        let client = ctx.data::<TiledClient>().unwrap();
        let headers = ctx
            .data_opt::<Option<AuthHeader>>()
            .and_then(|a| a.as_ref())
            .map(|a| a.as_header_map());
        client.stream_events(None, headers)
    }

    async fn node_events(&self, ctx: &Context<'_>, node: String) -> impl Stream<Item = TiledEvent> {
        let client = ctx.data::<TiledClient>().unwrap();
        let headers = ctx
            .data_opt::<Option<AuthHeader>>()
            .and_then(|a| a.as_ref())
            .map(|a| a.as_header_map());
        client.stream_events(Some(node), headers)
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_graphql::{EmptyMutation, Schema};
    use axum::Router;
    use axum::extract::WebSocketUpgrade;
    use axum::extract::ws::Message;
    use axum::http::{HeaderMap, Uri};
    use axum::routing::any;
    use futures_util::StreamExt;
    use rstest::rstest;
    use tokio::net::TcpListener;
    use tokio::sync::mpsc;
    use tokio::time;
    use url::Url;

    use super::*;
    use crate::clients::TiledClient;

    #[rstest]
    #[case::with_auth(Some("Bearer test-token"))]
    #[case::no_auth(None)]
    #[tokio::test]
    async fn test_stream_events_headers_parameterized(#[case] auth_token: Option<&str>) {
        let (tx, mut rx) = mpsc::channel(1);

        let app = Router::new().fallback(any(
            move |headers: HeaderMap, ws: WebSocketUpgrade| async move {
                let _ = tx.send(headers).await;
                ws.on_upgrade(|_| async move {})
            },
        ));

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let url = Url::parse(&format!("http://{addr}")).unwrap();
        let client = TiledClient::new(url);

        let mut headers = HeaderMap::new();
        if let Some(token) = auth_token {
            headers.insert("Authorization", token.parse().unwrap());
        }

        let stream = client.stream_events(None, Some(headers));
        tokio::pin!(stream);

        let _ = time::timeout(Duration::from_secs(5), stream.next()).await;

        let captured_headers = time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("Timeout waiting for headers")
            .expect("Channel closed");

        if let Some(token) = auth_token {
            assert_eq!(captured_headers.get("Authorization").unwrap(), token);
        } else {
            assert!(captured_headers.get("Authorization").is_none());
        }
    }

    #[rstest::rstest]
    #[case::root(
        "subscription { events { ... on ContainerSchema { version } } }",
        "/api/v1/stream/single/",
        42,
        "events"
    )]
    #[case::node(
        "subscription { nodeEvents(node: \"foo\") { ... on ContainerSchema { version } } }",
        "/api/v1/stream/single/foo",
        43,
        "nodeEvents"
    )]
    #[tokio::test]
    async fn test_subscription_events_parameterized(
        #[case] query: &str,
        #[case] expected_path: &str,
        #[case] expected_version: u32,
        #[case] result_key: &str,
    ) {
        let (tx, mut rx) = mpsc::channel(1);

        let app = Router::new().fallback(any(move |uri: Uri, ws: WebSocketUpgrade| async move {
            let path = uri.path().to_string();
            let root = path.ends_with("/api/v1/stream/single/");
            ws.on_upgrade(move |mut socket| async move {
                let version = if root { 42 } else { 43 };
                let event = serde_json::json!({
                    "type": "container-schema",
                    "version": version
                });
                let bin = rmp_serde::to_vec(&event).unwrap();
                socket.send(Message::Binary(bin.into())).await.unwrap();
                let _ = tx.send(path).await;
            })
        }));

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let url = Url::parse(&format!("http://{addr}")).unwrap();
        let client = TiledClient::new(url);

        let schema = Schema::build(crate::model::TiledQuery, EmptyMutation, TiledSubscription)
            .data(client)
            .finish();

        let stream = schema.execute_stream(query);
        let mut stream = Box::pin(stream);
        let res = time::timeout(Duration::from_secs(5), stream.next())
            .await
            .expect("Timeout waiting for events")
            .unwrap();

        assert_eq!(
            res.data,
            async_graphql::value!({ result_key: { "version": expected_version } })
        );

        let path = time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("Timeout waiting for path")
            .unwrap();

        assert_eq!(path, expected_path);
    }
}
