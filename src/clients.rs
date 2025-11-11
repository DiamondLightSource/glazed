use std::fmt;

use reqwest::Url;
use serde::de::DeserializeOwned;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::model::{app, event_stream, run};

pub type ClientResult<T> = Result<T, ClientError>;

pub struct TiledClient {
    pub address: Url,
}

impl TiledClient {
    #[instrument(skip(self))]
    async fn request<T: DeserializeOwned>(&self, endpoint: &str) -> ClientResult<T> {
        info!("Requesting from tiled: {}", endpoint);
        let url = self.address.join(endpoint)?;
        let response = reqwest::get(url).await?.error_for_status()?;
        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| ClientError::InvalidResponse(e, body))
    }
    pub async fn app_metadata(&self) -> ClientResult<app::AppMetadata> {
        self.request("/api/v1/").await
    }
    pub async fn run_metadata(&self, id: Uuid) -> ClientResult<run::RunMetadataRoot> {
        self.request(&format!("/api/v1/metadata/{id}")).await
    }
    pub async fn event_stream_metadata(
        &self,
        id: Uuid,
        stream: String,
    ) -> ClientResult<event_stream::EventStreamMetadataRoot> {
        self.request(&format!("/api/v1/metadata/{id}/{stream}"))
            .await
    }
}

#[derive(Debug)]
pub enum ClientError {
    InvalidPath(url::ParseError),
    ServerError(reqwest::Error),
    InvalidResponse(serde_json::Error, String),
}
impl From<url::ParseError> for ClientError {
    fn from(err: url::ParseError) -> ClientError {
        ClientError::InvalidPath(err)
    }
}
impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> ClientError {
        ClientError::ServerError(err)
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::InvalidPath(err) => write!(f, "Invalid URL path: {}", err),
            ClientError::ServerError(err) => write!(f, "Tiled server error: {}", err),
            ClientError::InvalidResponse(err, actual) => {
                write!(f, "Invalid response: {err}, response: {actual}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, Value};
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
    async fn test_server_unavailable() {
        let schema = build_schema("http://tiled.example.com");
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
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(503);
            })
            .await;
        let schema = build_schema(&server.base_url());
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
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200).body("{}");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let response = schema.execute("{appMetadata { apiVersion } }").await;

        assert_eq!(response.data, Value::Null);
        assert_eq!(response.errors.len(), 1);
        assert_eq!(
            response.errors[0].message,
            "Invalid response: missing field `api_version` at line 1 column 2, response: {}"
        );
        mock.assert();
    }
}
