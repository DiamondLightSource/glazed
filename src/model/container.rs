use serde::Deserialize;
use serde_json::Value;

use crate::model::event_stream;
use crate::model::run::{self, Start};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum ContainerMetadata {
    Run(Box<run::RunMetadata>),
    EventStream(event_stream::EventStreamMetadata),
}

impl ContainerMetadata {
    pub fn start_doc(&self) -> Option<&Start> {
        if let ContainerMetadata::Run(run) = self {
            Some(&run.start)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ContainerStructure {
    contents: Value,
    count: i64,
}
