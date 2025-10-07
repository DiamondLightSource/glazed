use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub id: String,
    pub attributes: Attributes,
    pub links: Links,
    pub meta: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<Spec>,
    pub metadata: Metadata,
    pub structure: Structure,
    pub access_blob: AccessBlob,
    pub sorting: Vec<Sorting>,
    pub data_sources: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    pub name: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub start: Start,
    pub stop: Stop,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Start {
    pub uid: String,
    pub time: f64,
    pub versions: Versions,
    pub instrument: String,
    pub instrument_session: String,
    pub data_session_directory: String,
    pub scan_file: String,
    pub scan_id: i64,
    pub plan_type: String,
    pub plan_name: String,
    pub detectors: Vec<String>,
    pub num_points: i64,
    pub num_intervals: i64,
    pub plan_args: PlanArgs,
    pub hints: Hints,
    pub shape: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Versions {
    pub ophyd: String,
    pub ophyd_async: String,
    pub bluesky: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanArgs {
    pub detectors: Vec<String>,
    pub num: i64,
    pub delay: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hints {
    pub dimensions: Vec<(Vec<String>, String)>,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub uid: String,
    pub time: f64,
    pub run_start: String,
    pub exit_status: String,
    pub reason: String,
    pub num_events: NumEvents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumEvents {
    pub primary: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    pub contents: Value,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessBlob {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sorting {
    pub key: String,
    pub direction: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub search: String,
    pub full: String,
}
