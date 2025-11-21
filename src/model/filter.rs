use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use crate::model::{node, run};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct FilterRun {
    pub data: Vec<FilterData>,
    pub error: Value,
    pub links: Option<node::Links>,
    pub meta: Value,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct FilterData {
    pub id: Uuid, // run_uuid
    pub attributes: FilterAttributes,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct FilterAttributes {
    pub metadata: run::RunMetadata,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
// pub struct FilterAttributes {
//     pub metadata: Vec<String>;
// }
