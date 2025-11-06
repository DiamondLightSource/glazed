use std::fmt;

use axum::http::HeaderMap;
use reqwest::Url;
use serde::de::DeserializeOwned;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::model::app_metadata::AppMetadata;
use crate::model::container::Container;
use crate::model::metadata::Root;

pub type ClientResult<T> = Result<T, ClientError>;

pub struct TiledClient {
    pub address: Url,
}

impl TiledClient {
    #[instrument(skip(self))]
    async fn request<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>,
    ) -> ClientResult<T> {
        info!("Requesting from tiled: {}", endpoint);
        let url = self.address.join(endpoint)?;
        let client = reqwest::Client::new();
        let request = match headers {
            Some(headers) => client.get(url).headers(headers),
            None => client.get(url),
        };
        let response = request.send().await?.error_for_status()?;
        let body = response.text().await?;

        serde_json::from_str(&body).map_err(|e| ClientError::InvalidResponse(e, body))
    }
    pub async fn app_metadata(&self) -> ClientResult<AppMetadata> {
        self.request::<AppMetadata>("/api/v1/", None).await
    }
    pub async fn run_metadata(&self, id: Uuid) -> ClientResult<Root> {
        self.request::<Root>(&format!("/api/v1/metadata/{id}"), None)
            .await
    }
    pub async fn container(&self, id: Uuid, path: Option<String>) -> ClientResult<Container> {
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/json".parse().unwrap());

        let endpoint = match path {
            Some(path) => &format!("api/v1/container/full/{id}/{path}"),
            None => &format!("api/v1/container/full/{id}"),
        };

        self.request::<Container>(endpoint, Some(headers)).await
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
