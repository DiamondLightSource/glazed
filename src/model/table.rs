use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::node;

pub type Table = HashMap<String, Vec<Value>>;

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
// pub struct TableAttributes {
//     pub ancestors: Vec<Value>,
//     pub specs: Option<Vec<Value>>,
//     pub metadata: HashMap<String, Value>,
//     #[serde(flatten)]
//     pub structure: node::Structure,
//     pub access_blob: Value,
//     pub sorting: Option<Vec<node::Sorting>>,
//     pub data_sources: Option<Vec<node::DataSource>>,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct TableStructure {
    pub arrow_schema: String,
    pub npartitions: i64,
    pub columns: Vec<Value>,
    pub resizable: bool,
}

#[cfg(test)]
mod tests {
    use crate::model::table;
    use crate::test_utils::assert_readable_as;

    // #[test]
    // fn table_metadata() {
    //     assert_readable_as::<table::TableMetadataRoot>("resources/metadata_table.json");
    // }
    // #[test]
    // fn table_full() {
    //     assert_readable_as::<table::Table>("resources/table_full.json");
    // }
}
