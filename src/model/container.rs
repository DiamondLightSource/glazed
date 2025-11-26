use async_graphql::{SimpleObject, Union};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::{event_stream, node, run};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Container {
    pub contents: Value,
    pub metadata: Value,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
// pub struct ContainerAttributes {
//     pub ancestors: Vec<Value>,
//     pub specs: Vec<ContainerSpecs>,
//     pub metadata: ContainerMetadata,
//     #[serde(flatten)]
//     pub structure: node::Structure,
//     pub access_blob: Value,
//     pub sorting: Vec<node::Sorting>,
//     pub data_sources: Option<Vec<node::DataSource>>,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ContainerSpecs {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum ContainerMetadata {
    Run(run::RunMetadata),
    EventStream(event_stream::EventStreamMetadata),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ContainerStructure {
    pub contents: Value,
    pub count: i64,
}
