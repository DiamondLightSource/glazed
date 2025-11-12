use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::node;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct AppMetadata {
    pub api_version: i64,
    pub library_version: String,
    pub queries: Vec<String>,
    pub links: node::Links,
    pub meta: Value,
}

#[cfg(test)]
mod tests {
    use crate::model::app;
    use crate::test_utils::assert_readable_as;

    #[test]
    fn app_metadata() {
        assert_readable_as::<app::AppMetadata>("resources/metadata_app.json");
    }
}
