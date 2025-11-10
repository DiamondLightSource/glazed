use std::{collections::HashMap, fmt};

use axum::http::HeaderMap;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::model::{app, array, event_stream, run, table};

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
        let mut url = self.address.clone();
        url.set_path(endpoint);

        let client = reqwest::Client::new();
        let request = match headers {
            Some(headers) => client.get(url).headers(headers),
            None => client.get(url),
        };
        let response = request.send().await?.error_for_status()?;
        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| ClientError::InvalidResponse(e, body))
    }
    pub async fn app_metadata(&self) -> ClientResult<app::AppMetadata> {
        self.request("/api/v1/", None).await
    }
    pub async fn run_metadata(&self, id: Uuid) -> ClientResult<run::RunMetadataRoot> {
        self.request(&format!("/api/v1/metadata/{id}"), None).await
    }
    pub async fn event_stream_metadata(
        &self,
        id: Uuid,
        stream: String,
    ) -> ClientResult<event_stream::EventStreamMetadataRoot> {
        self.request(&format!("/api/v1/metadata/{id}/{stream}"), None)
            .await
    }
    pub async fn array_metadata(
        &self,
        id: Uuid,
        stream: String,
        array: String,
    ) -> ClientResult<array::ArrayMetadataRoot> {
        self.request(&format!("/api/v1/metadata/{id}/{stream}/{array}"), None)
            .await
    }
    pub async fn table_metadata(
        &self,
        id: Uuid,
        stream: String,
        table: String,
    ) -> ClientResult<table::TableMetadataRoot> {
        self.request(&format!("/api/v1/metadata/{id}/{stream}/{table}"), None)
            .await
    }
    pub async fn search_root(&self) -> ClientResult<run::RunRoot> {
        self.request("/api/v1/search/", None).await
    }
    pub async fn search_run_container(
        &self,
        id: Uuid,
    ) -> ClientResult<event_stream::EventStreamRoot> {
        self.request(&format!("/api/v1/search/{id}"), None).await
    }
    pub async fn table(
        &self,
        id: Uuid,
        stream: String,
        table: String,
    ) -> ClientResult<HashMap<String, Vec<Value>>> {
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/json".parse().unwrap());

        self.request(
            &format!("/api/v1/table/full/{id}/{stream}/{table}"),
            Some(headers),
        )
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
