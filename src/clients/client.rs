use core::fmt;
use std::result;

use crate::schemas::tiled_metadata::Metadata;

// #[derive(Debug, Clone)]
#[derive(Debug)]
pub enum RequestError {
    Parse(url::ParseError),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}
impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// pub type ClientResult<T> = result::Result<T, RequestError>;

pub trait Client {
    fn metadata(&self) -> impl Future<Output = Result<Metadata, RequestError>> + Send;
}