use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::{container, node};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMetadataRoot {
    pub data: EventStreamData,
    pub error: Value,
    pub links: Option<node::Links>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamData {
    pub id: String,
    pub attributes: EventStreamContainerAttributes,
    pub links: node::Links,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamContainerAttributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<container::Specs>,
    pub metadata: EventStreamMetadata,
    pub structure: container::Structure,
    pub access_blob: Value,
    pub sorting: Option<Vec<container::Sorting>>,
    pub data_sources: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMetadata {
    configuration: HashMap<String, HashMap<String, Value>>,
    data_keys: HashMap<String, HashMap<String, Value>>,
    time: f64,
    uid: Uuid,
    hints: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, value};
    use httpmock::MockServer;
    use url::Url;
    use uuid::Uuid;

    use crate::TiledQuery;
    use crate::clients::TiledClient;

    fn build_schema(url: &str) -> Schema<TiledQuery, EmptyMutation, EmptySubscription> {
        Schema::build(
            TiledQuery(TiledClient {
                address: Url::parse(url).unwrap(),
            }),
            EmptyMutation,
            EmptySubscription,
        )
        .finish()
    }
    #[tokio::test]
    async fn event_stream_metadata() {
        let id = Uuid::parse_str("4866611f-e6d9-4517-bedf-fc5526df57ad").unwrap();
        let stream = "primary";
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path(format!("/api/v1/metadata/{id}/{stream}"));
                then.status(200)
                    .body_from_file("resources/event_stream_metadata.json");
            })
            .await;
        let schema = build_schema(&server.base_url());

        let query = r#"{eventStreamMetadata(id: "4866611f-e6d9-4517-bedf-fc5526df57ad", stream: "primary") {data {id attributes {specs {name version}}}}}"#;
        let exp = value!({
            "eventStreamMetadata": {
              "data": {
                "id": "primary",
                "attributes": {
                  "specs": [
                    {
                      "name": "BlueskyEventStream",
                      "version": "3.0"
                    },
                    {
                      "name": "composite",
                      "version": null
                    }
                  ]
                }
              }
        }});
        let response = schema.execute(query).await;

        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
}
