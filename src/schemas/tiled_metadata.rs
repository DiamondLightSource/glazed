// Auto-generated with JSON to serde tool

use async_graphql::SimpleObject;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "api_version")]
    pub api_version: i64,
    #[serde(rename = "library_version")]
    pub library_version: String,
    pub queries: Vec<String>,
    pub links: Links,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub documentation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "root_path")]
    pub root_path: String,
}
