use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Links {
    #[serde(rename = "self")]
    #[graphql(name = "self")]
    pub self_field: String,
    pub documentation: Option<String>,
    pub search: Option<String>,
    pub full: Option<String>,
}
