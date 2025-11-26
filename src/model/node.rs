use std::collections::HashMap;

use async_graphql::{Enum, OutputType, SimpleObject, Union};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::{array, container, table};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub data: Vec<Data>,
    pub error: Value,
    pub links: Option<Links>,
    pub meta: Value,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct MetadataRoot {
//     pub data: Data,
//     pub error: Value,
//     pub links: Option<Links>,
//     pub meta: Value,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub attributes: NodeAttributes,
    pub links: Links,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Spec {
    name: String,
    version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "structure_family", rename_all = "lowercase")]
pub enum NodeAttributes {
    Container(Attributes<container::ContainerMetadata, container::ContainerStructure>),
    Array(Attributes<HashMap<String, Value>, array::ArrayStructure>),
    Table(Attributes<HashMap<String, Value>, table::TableStructure>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes<Meta, S> {
    pub ancestors: Vec<String>,
    pub specs: Vec<Spec>,
    pub metadata: Meta,
    pub structure: S,
    pub access_blob: Value,
    pub sorting: Option<Vec<Sorting>>,
    pub data_sources: Option<Vec<DataSource<S>>>,
}

// #[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(
//     rename_all = "lowercase",
//     tag = "structure_family",
//     content = "attributes"
// )]
// pub enum Attributes {
//     Array(array::ArrayAttributes),
//     Container(container::ContainerAttributes),
//     Table(table::TableAttributes),
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sorting {
    pub key: String,
    pub direction: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(tag = "structure_family")]
pub struct DataSource<S> {
    pub structure: S,
    pub id: Option<u64>,
    pub mimetype: Option<String>,
    pub parameters: HashMap<String, Value>,
    pub assets: Vec<Asset>,
    management: Management,
}

// #[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(
//     rename_all = "lowercase",
//     tag = "structure_family",
//     content = "structure"
// )]
// pub enum Structure {
//     Array(array::ArrayStructure),
//     //Awkward(AwkwardSructure),
//     Container(container::ContainerStructure),
//     //Sparse(SparseStructure),
//     Table(table::TableStructure),
// }

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
    pub data_uri: String,
    is_directory: bool,
    parameter: Option<String>,
    num: Option<i64>,
    pub id: Option<i64>,
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
