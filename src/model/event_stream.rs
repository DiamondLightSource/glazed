use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::{container, node};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamRoot {
    pub data: Vec<EventStreamData>,
    pub error: Value,
    pub links: Option<node::Links>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamMetadataRoot {
    pub data: EventStreamData,
    pub error: Value,
    pub links: Option<node::Links>,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamData {
    pub id: String,
    pub attributes: EventStreamContainerAttributes,
    pub links: node::Links,
    pub meta: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct EventStreamContainerAttributes {
    pub ancestors: Vec<Value>,
    pub specs: Vec<container::Specs>,
    pub metadata: EventStreamMetadata,
    #[serde(flatten)]
    pub structure: node::Structure,
    pub access_blob: Value,
    pub sorting: Vec<node::Sorting>,
    pub data_sources: Option<Vec<node::DataSource>>,
}

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
    use crate::model::{container, event_stream};
    use crate::test_utils::assert_readable_as;

    #[test]
    fn event_stream_metadata() {
        assert_readable_as::<event_stream::EventStreamMetadataRoot>(
            "resources/metadata_event_stream.json",
        );
    }
    #[test]
    fn search_run_container_for_event_stream_containers() {
        assert_readable_as::<event_stream::EventStreamRoot>("resources/search_run_container.json");
    }
    #[test]
    fn container_full() {
        assert_readable_as::<container::Container>("resources/container_event_stream.json");
    }
}
