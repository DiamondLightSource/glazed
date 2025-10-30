use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::model::common::Links;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct AppMetadata {
    pub api_version: i64,
    pub library_version: String,
    pub queries: Vec<String>,
    pub links: Links,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Meta {
    pub root_path: String,
}
