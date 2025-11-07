use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ArrayData {
    pub id: String,
    pub attributes: ArrayAttributes,
    pub links: ArrayLinks,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ArrayAttributes {
    pub ancestors: Vec<Value>,
    pub structure_family: String,
    pub specs: Option<Vec<Value>>,
    pub metadata: HashMap<String, Value>,
    pub structure: ArrayStructure,
    pub access_blob: Value,
    pub sorting: Value,
    pub data_sources: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ArrayStructure {
    data_type: DataType,
    chunks: Value,
    shape: Value,
    dims: Value,
    resizable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct DataType {
    endianness: String,
    kind: String,
    itemsize: i64,
    dt_units: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ArrayLinks {
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub full: Option<String>,
    pub block: Option<String>,
}
