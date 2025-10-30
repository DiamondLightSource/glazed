use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::common::Links;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Data {
    pub id: Uuid,
    pub attributes: Attributes,
    pub links: Option<Links>,
    pub meta: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Attributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<Spec>,
    pub metadata: Metadata,
    pub structure: Structure,
    pub sorting: Vec<Sorting>,
    pub data_sources: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Spec {
    pub name: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Metadata {
    pub start: Start,
    pub stop: Stop,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Versions {
    pub ophyd: String,
    pub ophyd_async: String,
    pub bluesky: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Hints {
    pub dimensions: Vec<HintDimension>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, SimpleObject)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Stop {
    pub uid: Uuid,
    pub time: f64,
    pub run_start: Uuid,
    pub exit_status: String,
    pub reason: String,
    pub num_events: NumEvents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct NumEvents {
    pub primary: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Structure {
    pub contents: Value,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Sorting {
    pub key: String,
    pub direction: i64,
}
