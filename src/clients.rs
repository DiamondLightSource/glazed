use std::fmt;

use reqwest::Url;
use serde::de::DeserializeOwned;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::model::app;
use crate::model::event_stream;
use crate::model::run;

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
    pub async fn search_container(
        &self,
        id: Uuid,
        name: Option<String>,
    ) -> ClientResult<SearchRoot> {
        let endpoint = match name {
            Some(name) => &format!("/api/v1/search/{id}/{name}"),
            None => &format!("/api/v1/search/{id}"),
        };

        self.request(endpoint).await
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
