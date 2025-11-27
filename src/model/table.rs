use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Table = HashMap<String, Vec<Value>>;

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
