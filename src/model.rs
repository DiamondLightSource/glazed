pub(crate) mod metadata;

use async_graphql::Object;

use crate::clients::{Client, ClientError, TiledClient};

pub(crate) struct TiledQuery(pub TiledClient);

#[Object]
impl TiledQuery {
    async fn metadata(&self) -> async_graphql::Result<metadata::Metadata, ClientError> {
        self.0.metadata().await
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use httpmock::MockServer;
    use url::Url;

    use crate::TiledQuery;
    use crate::clients::TiledClient;

    #[tokio::test]
    async fn test_api_version_query() {
        let mock_server = MockServer::start();

        let mock = mock_server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200)
                    .body_from_file("resources/tiled_metadata.json");
            })
            .await;

        let schema = Schema::build(
            TiledQuery(TiledClient {
                address: Url::parse(&mock_server.base_url()).unwrap(),
            }),
            EmptyMutation,
            EmptySubscription,
        )
        .finish();

        let response = schema.execute("{metadata { apiVersion } }").await;

        assert_eq!(response.data.to_string(), "{metadata: {apiVersion: 0}}");
        mock.assert();
    }
}
