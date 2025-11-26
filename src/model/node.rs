use std::collections::HashMap;

use async_graphql::{Enum, SimpleObject, Union};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::{array, container, table};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Root {
    pub data: Vec<Data>,
    pub error: Value,
    pub links: Option<Links>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct MetadataRoot {
    pub data: Data,
    pub error: Value,
    pub links: Option<Links>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Data {
    pub id: String,
    #[serde(flatten)]
    pub attributes: Attributes,
    pub links: Links,
    pub meta: Value,
}

#[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    rename_all = "lowercase",
    tag = "structure_family",
    content = "attributes"
)]
pub enum Attributes {
    Array(array::ArrayAttributes),
    Container(container::ContainerAttributes),
    Table(table::TableAttributes),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Sorting {
    pub key: String,
    pub direction: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct DataSource {
    #[serde(flatten)]
    pub structure: Structure,
    pub id: Option<u64>,
    pub mimetype: Option<String>,
    pub parameters: HashMap<String, Value>,
    pub assets: Vec<Asset>,
    management: Management,
}

#[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    rename_all = "lowercase",
    tag = "structure_family",
    content = "structure"
)]
pub enum Structure {
    Array(array::ArrayStructure),
    //Awkward(AwkwardSructure),
    Container(container::ContainerStructure),
    //Sparse(SparseStructure),
    Table(table::TableStructure),
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
    pub block: Option<String>,
    pub partition: Option<String>,
}
