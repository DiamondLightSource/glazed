use async_graphql::SimpleObject;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Deserialize, SimpleObject)]
pub struct AppMetadata {
    pub api_version: i64,
    pub library_version: String,
    pub queries: Vec<String>,
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
