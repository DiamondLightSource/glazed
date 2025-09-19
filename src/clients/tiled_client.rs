use crate::{
    clients::client::{Client, ClientResult},
    schemas::tiled_metadata::Metadata,
};

use reqwest::Url;
use serde::de::DeserializeOwned;

pub struct TiledClient;

impl TiledClient {
    async fn request<T: DeserializeOwned>(&self, endpoint: &str) -> ClientResult<T> {
        println!("Requesting data from tiled");

        let mut path: String = String::from("http://127.0.0.1:8000");
        path.push_str(endpoint);
        let path = Url::parse(&path)?;

        let req = reqwest::get(path).await?;
        let json = req.json().await?;
        Ok(serde_json::from_value(json)?)
    }
}
impl Client for TiledClient {
    async fn metadata(&self) -> ClientResult<Metadata> {
        self.request::<Metadata>("/api/v1/").await
    }
}
