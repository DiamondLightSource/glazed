use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::{container, node};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunRoot {
    pub data: Vec<RunData>,
    pub error: Value,
    pub links: Option<node::Links>,
    pub meta: Value,
}

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
    use crate::model::run;
    use crate::test_utils::assert_readable_as;

    #[test]
    fn run_metadata() {
        assert_readable_as::<run::RunMetadataRoot>("resources/metadata_run.json");
    }
    #[test]
    fn search_root_for_run_containers() {
        assert_readable_as::<run::RunRoot>("resources/search_root.json");
    }
}
