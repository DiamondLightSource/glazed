// Auto-generated with JSON to serde tool

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Metadata {
    pub api_version: i64,
    pub library_version: String,
    pub queries: Vec<String>,
    pub links: Links,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub documentation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Meta {
    pub root_path: String,
}
