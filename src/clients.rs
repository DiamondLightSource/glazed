use std::fmt;

use axum::http::HeaderMap;
#[cfg(test)]
use httpmock::MockServer;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::model::{app, container, node, table};

pub type ClientResult<T> = Result<T, ClientError>;

pub struct TiledClient {
    client: Client,
    address: Url,
}

impl TiledClient {
    pub fn new(address: Url) -> Self {
        Self {
            client: Client::new(),
            address,
        }
    }
    #[instrument(skip(self))]
    async fn request<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>,
        query_params: Option<&[(&str, &str)]>,
    ) -> ClientResult<T> {
        info!("Requesting from tiled: {}", endpoint);
        let url = self.address.join(endpoint)?;

        let mut request = match headers {
            Some(headers) => self.client.get(url).headers(headers),
            None => self.client.get(url),
        };
        if let Some(params) = query_params {
            request = request.query(params);
        }
        info!("Querying: {request:?}");

        let response = request.send().await?.error_for_status()?;
        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| ClientError::InvalidResponse(e, body))
    }
    pub async fn app_metadata(&self) -> ClientResult<app::AppMetadata> {
        self.request("/api/v1/", None, None).await
    }
    pub async fn search<T: DeserializeOwned>(&self, path: &str) -> ClientResult<T> {
        self.request(path, None, None).await
    }
    // pub async fn run_metadata(&self, id: Uuid) -> ClientResult<node::MetadataRoot> {
    //     self.request(&format!("/api/v1/metadata/{id}"), None, None)
    //         .await
    // }
    // pub async fn event_stream_metadata(
    //     &self,
    //     id: Uuid,
    //     stream: String,
    // ) -> ClientResult<node::MetadataRoot> {
    //     self.request(&format!("/api/v1/metadata/{id}/{stream}"), None, None)
    //         .await
    // }
    // pub async fn array_metadata(
    //     &self,
    //     id: Uuid,
    //     stream: String,
    //     array: String,
    // ) -> ClientResult<node::MetadataRoot> {
    //     self.request(
    //         &format!("/api/v1/metadata/{id}/{stream}/{array}"),
    //         None,
    //         Some(&[("include_data_sources", "true")]),
    //     )
    //     .await
    // }
    // pub async fn table_metadata(
    //     &self,
    //     id: Uuid,
    //     stream: String,
    //     table: String,
    // ) -> ClientResult<node::MetadataRoot> {
    //     self.request(
    //         &format!("/api/v1/metadata/{id}/{stream}/{table}"),
    //         None,
    //         Some(&[("include_data_sources", "true")]),
    //     )
    //     .await
    // }
    // pub async fn table_full(
    //     &self,
    //     id: Uuid,
    //     stream: String,
    //     table: String,
    // ) -> ClientResult<table::Table> {
    //     let mut headers = HeaderMap::new();
    //     headers.insert("accept", "application/json".parse().unwrap());

    //     self.request(
    //         &format!("/api/v1/table/full/{id}/{stream}/{table}"),
    //         Some(headers),
    //         None,
    //     )
    //     .await
    // }
    // pub async fn search_root(&self) -> ClientResult<node::Root> {
    //     self.request("/api/v1/search/", None, None).await
    // }
    // pub async fn search_run_container(&self, id: Uuid) -> ClientResult<node::Root> {
    //     self.request(&format!("/api/v1/search/{id}"), None, None)
    //         .await
    // }
    // pub async fn container_full(
    //     &self,
    //     id: Uuid,
    //     stream: Option<String>,
    // ) -> ClientResult<container::Container> {
    //     let mut headers = HeaderMap::new();
    //     headers.insert("accept", "application/json".parse().unwrap());

    //     let endpoint = match stream {
    //         Some(stream) => &format!("/api/v1/container/full/{id}/{stream}"),
    //         None => &format!("/api/v1/container/full/{id}"),
    //     };

    //     self.request(endpoint, Some(headers), None).await
    // }

    /// Create a new client for the given mock server
    #[cfg(test)]
    pub fn for_mock_server(server: &MockServer) -> Self {
        Self {
            // We're only in tests so panicking is fine
            address: server.base_url().parse().unwrap(),
            client: Client::new(),
        }
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
    use axum::http::HeaderMap;
    use httpmock::MockServer;

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
        let client = TiledClient::for_mock_server(&server);
        assert_eq!(
            client
                .request::<Vec<u8>>("/demo/api", None, None)
                .await
                .unwrap(),
            vec![1, 2, 3]
        );
        mock.assert();
    }
    #[tokio::test]
    async fn request_with_headers() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path("/demo/api")
                    .header("api-key", "foo");
                then.status(200).body("[1,2,3]");
            })
            .await;
        let client = TiledClient::for_mock_server(&server);
        let mut headers = HeaderMap::new();
        headers.insert("api-key", "foo".parse().unwrap());

        assert_eq!(
            client
                .request::<Vec<u8>>("/demo/api", Some(headers), None)
                .await
                .unwrap(),
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
        let client = TiledClient::for_mock_server(&server);
        let response = client.app_metadata().await.unwrap();

        assert_eq!(response.api_version, 0);
        mock.assert();
    }
    #[tokio::test]
    async fn server_unavailable() {
        let client = TiledClient::new("http://non-existent.example.com".parse().unwrap());
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

        let client = TiledClient::for_mock_server(&server);
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

        let client = TiledClient::for_mock_server(&server);
        let response = client.app_metadata().await;

        let Err(ClientError::InvalidResponse(err, _)) = response else {
            panic!("Expected InvalidResponse but got {response:?}");
        };

        assert!(err.is_data());
        mock.assert();
    }
}
