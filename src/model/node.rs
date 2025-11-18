use std::collections::HashMap;

use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Links {
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub self_field: String,
    pub documentation: Option<String>,
    pub first: Option<String>,
    pub last: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub search: Option<String>,
    pub full: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct DataSource {
    pub structure_family: StructureFamily,
    pub structure: Value,
    pub id: Option<u64>,
    pub mimetype: Option<String>,
    pub parameters: HashMap<String, Value>,
    pub assets: Vec<Asset>,
    management: Management,
}

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StructureFamily {
    Array,
    Awkward,
    Container,
    Sparse,
    Table,
}

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Management {
    External,
    Immutable,
    Locked,
    Writable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Asset {
    data_uri: String,
    is_directory: bool,
    parameter: Option<String>,
    num: Option<i64>,
    id: Option<i64>,
}
