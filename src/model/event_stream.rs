use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMetadata {
    configuration: HashMap<String, HashMap<String, Value>>,
    data_keys: HashMap<String, HashMap<String, Value>>,
    time: f64,
    uid: Uuid,
    hints: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use crate::model::node;
    use crate::test_utils::assert_readable_as;

    #[test]
    fn search_run_container_for_event_stream_containers() {
        assert_readable_as::<node::Root>("resources/search_run_container.json");
    }
}
