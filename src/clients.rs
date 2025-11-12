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
    pub async fn search_root(&self) -> ClientResult<run::RunRoot> {
        self.request("/api/v1/search/").await
    }
    pub async fn search_run_container(
        &self,
        id: Uuid,
    ) -> ClientResult<event_stream::EventStreamRoot> {
        self.request(&format!("/api/v1/search/{id}")).await
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
    use httpmock::MockServer;
    use url::Url;

    use crate::clients::{ClientError, TiledClient};

    #[tokio::test]
    async fn request() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/demo/api");
                then.status(200).body("[1,2,3]");
            })
            .await;
        let client = TiledClient {
            address: Url::parse(&server.base_url()).unwrap(),
        };
        assert_eq!(
            client.request::<Vec<u8>>("/demo/api").await.unwrap(),
            vec![1, 2, 3]
        );
        mock.assert();
    }

    #[tokio::test]
    async fn request_app_metadata() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200)
                    .body_from_file("resources/metadata_app.json");
            })
            .await;
        let client = TiledClient {
            address: Url::parse(&server.base_url()).unwrap(),
        };
        let response = client.app_metadata().await.unwrap();

        assert_eq!(response.api_version, 0);
        mock.assert();
    }
    #[tokio::test]
    async fn server_unavailable() {
        let client = TiledClient {
            address: Url::parse("http://tiled.example.com").unwrap(),
        };
        let response = client.app_metadata().await;

        let Err(ClientError::ServerError(err)) = response else {
            panic!("Expected ServerError but got {response:?}");
        };
        assert!(
            err.is_connect(),
            "Expected connection error but got {err:?}"
        );
    }

    #[tokio::test]
    async fn internal_tiled_error() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(503);
            })
            .await;

        let client = TiledClient {
            address: Url::parse(&server.base_url()).unwrap(),
        };
        let response = client.app_metadata().await;

        let Err(ClientError::ServerError(err)) = response else {
            panic!("Expected ServerError but got {response:?}");
        };

        assert!(err.is_status());
        assert!(
            err.status().is_some_and(|x| x == 503),
            "Expected 503 but was {:?}",
            err.status()
        );
        mock.assert();
    }

    #[tokio::test]
    async fn invalid_server_response() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200).body("{}");
            })
            .await;

        let client = TiledClient {
            address: Url::parse(&server.base_url()).unwrap(),
        };
        let response = client.app_metadata().await;

        let Err(ClientError::InvalidResponse(err, _)) = response else {
            panic!("Expected InvalidResponse but got {response:?}");
        };

        assert!(err.is_data());
        mock.assert();
    }
}
