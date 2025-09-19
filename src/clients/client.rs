// use core::fmt;
// use std::error;

use std::{fmt, result};

use crate::schemas::tiled_metadata::Metadata;

// #[derive(Debug, Clone)]
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

// impl error::Error for ClientError {

// }

pub type ClientResult<T> = Result<T, ClientError>;

pub trait Client {
    fn metadata(&self) -> impl Future<Output = Result<Metadata, ClientError>> + Send;
}