use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::node;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ArrayAttributes {
    pub ancestors: Vec<Value>,
    pub specs: Option<Vec<Value>>,
    pub metadata: HashMap<String, Value>,
    #[serde(flatten)]
    pub structure: node::Structure,
    pub access_blob: Value,
    pub sorting: Option<Vec<node::Sorting>>,
    pub data_sources: Option<Vec<node::DataSource>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct ArrayStructure {
    data_type: DataType,
    chunks: Value,
    shape: Value,
    dims: Value,
    resizable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct DataType {
    endianness: String,
    kind: String,
    itemsize: i64,
    dt_units: Value,
}

#[cfg(test)]
mod tests {
    use crate::model::array;
    use crate::test_utils::assert_readable_as;

    #[test]
    fn array_metadata() {
        assert_readable_as::<array::ArrayMetadataRoot>("resources/metadata_array.json");
    }
}
