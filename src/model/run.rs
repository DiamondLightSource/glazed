use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RunMetadata {
    pub start: Start,
    stop: Option<Stop>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Start {
    uid: Uuid,
    time: f64,
    versions: Versions,
    instrument: String,
    instrument_session: String,
    data_session_directory: Option<String>,
    scan_file: Option<String>,
    pub scan_id: i64,
    plan_type: String,
    plan_name: String,
    detectors: Vec<String>,
    motors: Option<Vec<String>>,
    num_points: i64,
    num_intervals: i64,
    plan_args: HashMap<String, Value>,
    hints: Hints,
    shape: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Versions {
    ophyd: String,
    ophyd_async: String,
    bluesky: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Hints {
    dimensions: Vec<HintDimension>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HintDimension {
    axes: Vec<String>,
    stream: String,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Stop {
    uid: Uuid,
    time: f64,
    run_start: Uuid,
    exit_status: String,
    reason: String,
    num_events: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use crate::model::node;
    use crate::test_utils::assert_readable_as;

    #[test]
    fn search_root_for_run_containers() {
        assert_readable_as::<node::Root>("resources/search_root.json");
    }
}
