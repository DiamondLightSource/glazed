pub(crate) mod app_metadata;
pub(crate) mod common;
pub(crate) mod metadata;

use async_graphql::Object;
use uuid::Uuid;

use crate::clients::{Client, ClientError, TiledClient};

pub(crate) struct TiledQuery(pub TiledClient);

#[Object]
impl TiledQuery {
    async fn app_metadata(&self) -> async_graphql::Result<app_metadata::AppMetadata, ClientError> {
        self.0.app_metadata().await
    }
    async fn run_metadata(&self, id: Uuid) -> async_graphql::Result<metadata::Root, ClientError> {
        self.0.run_metadata(id).await
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, Value, value};
    use httpmock::MockServer;
    use url::Url;
    use uuid::Uuid;

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

        let response = schema.execute("{appMetadata { apiVersion } }").await;

        assert_eq!(response.data.to_string(), "{appMetadata: {apiVersion: 0}}");
        assert_eq!(response.errors, &[]);
        mock.assert();
    }

    #[tokio::test]
    async fn test_server_unavailable() {
        let schema = Schema::build(
            TiledQuery(TiledClient {
                address: Url::parse("http://tiled.example.com").unwrap(),
            }),
            EmptyMutation,
            EmptySubscription,
        )
        .finish();

        let response = schema.execute("{appMetadata { apiVersion } }").await;
        assert_eq!(response.data, Value::Null);
        assert_eq!(
            response.errors[0].message,
            "Tiled server error: error sending request for url (http://tiled.example.com/api/v1/)"
        );
        assert_eq!(response.errors.len(), 1);
    }

    #[tokio::test]
    async fn test_internal_tiled_error() {
        let mock_server = MockServer::start();
        let mock = mock_server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(503);
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

        let response = schema.execute("{appMetadata { apiVersion } }").await;
        let actual = &response.errors[0].message;
        let expected =
            "Tiled server error: HTTP status server error (503 Service Unavailable) for url";
        assert_eq!(response.data, Value::Null);
        assert!(
            actual.starts_with(expected),
            "Unexpected error: {actual} \nExpected: {expected} [...]"
        );
        assert_eq!(response.errors.len(), 1);
        mock.assert();
    }

    #[tokio::test]
    async fn test_invalid_server_response() {
        let mock_server = MockServer::start();
        let mock = mock_server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200).body("{}");
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

        let response = schema.execute("{appMetadata { apiVersion } }").await;
        assert_eq!(response.data, Value::Null);
        assert_eq!(response.errors.len(), 1);
        assert_eq!(
            response.errors[0].message,
            "Invalid response: missing field `api_version` at line 1 column 2, response: {}"
        );
        mock.assert();
    }

    #[tokio::test]
    async fn test_run_metadata_query() {
        let id = Uuid::parse_str("5d8f5c3e-0e00-4c5c-816d-70b4b0f41498").unwrap();
        let mock_server = MockServer::start();
        let mock = mock_server
            .mock_async(|when, then| {
                when.method("GET").path(format!("/api/v1/metadata/{id}"));
                then.status(200)
                    .body_from_file("resources/run_metadata.json");
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

        let query = r#"{ runMetadata(id: "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498") {data {id}}}"#;

        let response = schema.execute(query).await;

        let exp = value! ({
            "runMetadata": { "data": {"id": "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498"}}
        });

        mock.assert();
        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
    }
}
