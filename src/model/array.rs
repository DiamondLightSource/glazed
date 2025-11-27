use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
