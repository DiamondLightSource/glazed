use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::metadata::Metadata;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct SearchRoot {
    pub data: Vec<Data>,
    pub error: Value,
    pub links: Option<SearchLinks>,
    pub meta: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct MetadataRoot {
    pub data: Data,
    pub error: Value,
    pub links: Option<SearchLinks>,
    pub meta: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Data {
    pub id: String,
    pub attributes: Attributes,
    pub links: DataLinks,
    pub meta: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Attributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Vec<Spec>,
    pub metadata: Metadata,
    pub structure: Structure,
    pub access_blob: Value,
    pub sorting: Vec<Sorting>,
    pub data_sources: Value,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct SearchLinks {
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub self_field: String,
    pub first: Option<String>,
    pub last: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct DataLinks {
    pub documentation: Option<String>,
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub self_field: String,
    pub search: Option<String>,
    pub full: Option<String>,
}
