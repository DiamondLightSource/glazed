use async_graphql::{SimpleObject, Union};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::event_stream;
use crate::model::run::{self, Start};

#[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum ContainerMetadata {
    Run(Box<run::RunMetadata>),
    EventStream(event_stream::EventStreamMetadata),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ContainerStructure {
    pub contents: Value,
    pub count: i64,
}
