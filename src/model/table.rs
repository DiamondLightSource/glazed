use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Table = HashMap<String, Vec<Value>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct TableStructure {
    pub arrow_schema: String,
    pub npartitions: i64,
    pub columns: Vec<Value>,
    pub resizable: bool,
}
