use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::{container, node};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunMetadataRoot {
    pub data: RunData,
    pub error: Value,
    pub links: Option<node::Links>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunData {
    pub id: Uuid,
    pub attributes: RunContainerAttributes,
    pub links: node::Links,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunContainerAttributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<container::Specs>,
    pub metadata: RunMetadata,
    pub structure: container::Structure,
    pub access_blob: Value,
    pub sorting: Vec<container::Sorting>,
    pub data_sources: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunMetadata {
    pub start: Start,
    pub stop: Option<Stop>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Start {
    pub uid: Uuid,
    pub time: f64,
    pub versions: Versions,
    pub instrument: String,
    pub instrument_session: String,
    pub data_session_directory: Option<String>,
    pub scan_file: Option<String>,
    pub scan_id: i64,
    pub plan_type: String,
    pub plan_name: String,
    pub detectors: Vec<String>,
    pub motors: Option<Vec<String>>,
    pub num_points: i64,
    pub num_intervals: i64,
    pub plan_args: HashMap<String, Value>,
    pub hints: Hints,
    pub shape: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Versions {
    pub ophyd: String,
    pub ophyd_async: String,
    pub bluesky: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Hints {
    pub dimensions: Vec<HintDimension>,
}

#[derive(Debug, Clone, PartialEq, Serialize, SimpleObject)]
pub struct HintDimension {
    pub axes: Vec<String>,
    pub stream: String,
}

impl<'de> Deserialize<'de> for HintDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (axes, stream) = <(Vec<String>, String)>::deserialize(deserializer)?;
        Ok(Self { axes, stream })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Stop {
    pub uid: Uuid,
    pub time: f64,
    pub run_start: Uuid,
    pub exit_status: String,
    pub reason: String,
    pub num_events: HashMap<String, Value>,
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
    async fn test_run_metadata_query() {
        let id = Uuid::parse_str("5d8f5c3e-0e00-4c5c-816d-70b4b0f41498").unwrap();
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path(format!("/api/v1/metadata/{id}"));
                then.status(200)
                    .body_from_file("resources/run_metadata.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let query = r#"{ runMetadata(id: "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498") {data {id}}}"#;
        let response = schema.execute(query).await;
        let exp = value! ({
            "runMetadata": { "data": {"id": "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498"}}
        });

        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
}
