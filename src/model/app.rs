use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::node;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct AppMetadata {
    pub api_version: i64,
    pub library_version: String,
    pub queries: Vec<String>,
    pub links: node::Links,
    pub meta: Value,
}
