use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMetadata {
    configuration: HashMap<String, HashMap<String, Value>>,
    data_keys: HashMap<String, HashMap<String, Value>>,
    time: i64,
    uid: Uuid,
    hints: HashMap<String, Value>,
}
