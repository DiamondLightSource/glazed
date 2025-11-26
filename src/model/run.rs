use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

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
    use crate::model::{container, run};
    use crate::test_utils::assert_readable_as;

    #[test]
    fn run_metadata() {
        assert_readable_as::<run::RunMetadataRoot>("resources/metadata_run.json");
    }
    #[test]
    fn search_root_for_run_containers() {
        assert_readable_as::<run::RunRoot>("resources/search_root.json");
    }
    #[test]
    fn container_full() {
        assert_readable_as::<container::Container>("resources/container_run.json");
    }
}
