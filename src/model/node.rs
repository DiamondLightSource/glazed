use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::run_metadata::RunMetadata;

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
    pub metadata: RunMetadata, // RunMd | EventStreamMd
    pub structure: Structure,
    pub access_blob: Value,
    pub sorting: Vec<Sorting>,
    pub data_sources: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Links {
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub self_field: String,
    pub documentation: Option<String>,
    pub search: Option<String>,
    pub full: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Spec {
    pub name: String,
    pub version: String,
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
