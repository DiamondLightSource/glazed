use std::collections::HashMap;

use async_graphql::Enum;
use serde::Deserialize;
use serde_json::Value;

use crate::model::{array, container, table};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Root {
    data: Vec<DataOption>,
    error: Value,
    meta: Value,
}

impl Root {
    pub fn data(&self) -> impl Iterator<Item = &Data> {
        self.data.iter().flat_map(DataOption::as_data)
    }
    pub fn into_data(self) -> impl Iterator<Item = Data> {
        self.data.into_iter().flat_map(DataOption::into_data)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum DataOption {
    Data(Data),
    Error(Value),
}

impl DataOption {
    fn as_data(&self) -> Option<&Data> {
        match self {
            Self::Data(data) => Some(data),
            Self::Error(_) => None,
        }
    }
    fn into_data(self) -> Option<Data> {
        match self {
            Self::Data(data) => Some(data),
            Self::Error(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Data {
    pub id: String,
    pub attributes: Box<NodeAttributes>,
    meta: Value,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "structure_family", rename_all = "lowercase")]
pub enum NodeAttributes {
    Container(Attributes<container::ContainerMetadata, container::ContainerStructure>),
    Array(Attributes<HashMap<String, Value>, array::ArrayStructure>),
    Table(Attributes<HashMap<String, Value>, table::TableStructure>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Attributes<Meta, S> {
    pub ancestors: Vec<String>,
    specs: Vec<Spec>,
    pub metadata: Meta,
    pub structure: S,
    access_blob: Value,
    sorting: Option<Vec<Sorting>>,
    pub data_sources: Option<Vec<DataSource<S>>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Spec {
    name: String,
    version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Sorting {
    key: String,
    direction: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DataSource<S> {
    structure: S,
    id: Option<u64>,
    mimetype: Option<String>,
    parameters: HashMap<String, Value>,
    pub assets: Vec<Asset>,
    management: Management,
}

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Management {
    External,
    Immutable,
    Locked,
    Writable,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Asset {
    pub data_uri: String,
    is_directory: bool,
    parameter: Option<String>,
    num: Option<i64>,
    pub id: Option<i64>,
}
