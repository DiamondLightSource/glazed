use std::fmt;

use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::model::app_metadata::AppMetadata;

pub trait Client {
    fn app_metadata(&self) -> impl Future<Output = Result<AppMetadata, ClientError>> + Send;
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct TiledClient {
    pub address: Url,
}

impl TiledClient {
    async fn request<T: DeserializeOwned>(&self, endpoint: &str) -> ClientResult<T> {
        println!("Requesting data from tiled");
        let url = self.address.join(endpoint)?;
        let response = reqwest::get(url).await?.error_for_status()?;
        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| ClientError::InvalidResponse(e, body))
    }
}
impl Client for TiledClient {
    async fn app_metadata(&self) -> ClientResult<AppMetadata> {
        self.request::<AppMetadata>("/api/v1/").await
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
