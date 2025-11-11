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
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, value};
    use httpmock::MockServer;
    use url::Url;

    use crate::TiledQuery;
    use crate::clients::TiledClient;

    fn build_schema(url: &str) -> Schema<TiledQuery, EmptyMutation, EmptySubscription> {
        Schema::build(
            TiledQuery(TiledClient {
                address: Url::parse(url).unwrap(),
            }),
            EmptyMutation,
            EmptySubscription,
        )
        .finish()
    }

    #[tokio::test]
    async fn test_api_version_query() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200)
                    .body_from_file("resources/tiled_metadata.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let response = schema.execute("{appMetadata { apiVersion } }").await;

        assert_eq!(response.data, value! {{"appMetadata": {"apiVersion": 0}}});
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
}
