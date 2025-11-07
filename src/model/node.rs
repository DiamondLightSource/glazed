use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::metadata::{EventStreamMetadata, RunMetadata};

// RunMd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunMdRoot {
    pub data: RunMdData,
    pub error: Value,
    pub links: Option<SearchLinks>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunMdData {
    pub id: String,
    pub attributes: RunMdAttributes,
    pub links: DataLinks,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct RunMdAttributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<Spec>,
    pub metadata: RunMetadata,
    pub structure: Structure,
    pub access_blob: Value,
    pub sorting: Vec<Sorting>,
    pub data_sources: Value,
}

// EventStreamMd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMdRoot {
    pub data: EventStreamMdData,
    pub error: Value,
    pub links: Option<SearchLinks>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMdData {
    pub id: String,
    pub attributes: EventStreamMdAttributes,
    pub links: DataLinks,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMdAttributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<Spec>,
    pub metadata: EventStreamMetadata,
    pub structure: Structure,
    pub access_blob: Value,
    pub sorting: Vec<Sorting>,
    pub data_sources: Value,
}

// search
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct SearchRoot {
    pub data: Vec<SearchData>,
    pub error: Value,
    pub links: Option<SearchLinks>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct SearchData {
    pub id: String,
    pub attributes: SearchAttributes,
    pub links: SearchLinks,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct SearchAttributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<Spec>,
    pub metadata: Value,  // Can be either Run or event
    pub structure: Value, // could be array/table etc. could be anything
    pub access_blob: Value,
    pub sorting: Option<Vec<Sorting>>,
    pub data_sources: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Spec {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Structure {
    pub contents: Value,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Sorting {
    pub key: String,
    pub direction: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct SearchLinks {
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub self_field: String,
    pub first: Option<String>,
    pub last: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct DataLinks {
    pub documentation: Option<String>,
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub self_field: String,
    pub search: Option<String>,
    pub full: Option<String>,
}
