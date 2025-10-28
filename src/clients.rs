use std::fmt;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::path::PathBuf;

use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::model::metadata::Metadata;

pub trait Client {
    fn metadata(&self) -> impl Future<Output = Result<Metadata, ClientError>> + Send;
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct TiledClient {
    pub address: Url,
}

impl TiledClient {
    async fn request<T: DeserializeOwned>(&self, endpoint: &str) -> ClientResult<T> {
        println!("Requesting data from tiled");

        let url = self.address.join(endpoint)?;

        let response = reqwest::get(url).await?;
        let json = response.json().await?;

        Ok(serde_json::from_value(json)?)
    }
}
impl Client for TiledClient {
    async fn metadata(&self) -> ClientResult<Metadata> {
        self.request::<Metadata>("/api/v1/").await
    }
}

#[cfg(test)]
pub struct MockTiledClient {
    pub dir_path: PathBuf,
}

#[cfg(test)]
impl MockTiledClient {
    async fn deserialize_from_file<T: DeserializeOwned>(&self, filename: &str) -> ClientResult<T> {
        println!("Requesting data from mock");

        let path = self.dir_path.join(filename);
        let file = File::open(&path)?;

        Ok(serde_json::from_reader(file)?)
    }
}
#[cfg(test)]
impl Client for MockTiledClient {
    async fn metadata(&self) -> ClientResult<Metadata> {
        self.deserialize_from_file("tiled_metadata.json").await
    }
}

#[derive(Debug)]
pub enum ClientError {
    Parse(url::ParseError),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}
impl From<url::ParseError> for ClientError {
    fn from(err: url::ParseError) -> ClientError {
        ClientError::Parse(err)
    }
}
impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> ClientError {
        ClientError::Reqwest(err)
    }
}
impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> ClientError {
        ClientError::Serde(err)
    }
}
impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> ClientError {
        ClientError::Io(err)
    }
}
impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::Parse(ref err) => write!(f, "Parse error: {}", err),
            ClientError::Reqwest(ref err) => write!(f, "Request error: {}", err),
            ClientError::Serde(ref err) => write!(f, "Serde error: {}", err),
            ClientError::Io(ref err) => write!(f, "IO Error: {}", err),
        }
    }
}
