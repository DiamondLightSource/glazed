use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMetadata {
    configuration: Value,
    data_keys: Value,
}
