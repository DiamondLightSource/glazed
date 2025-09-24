use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::clients::{Client, ClientResult};
use crate::schemas::tiled_metadata::Metadata;

pub struct TiledClient{
    pub address: String,
}

impl TiledClient {
    async fn request<T: DeserializeOwned>(&self, endpoint: &str) -> ClientResult<T> {
        println!("Requesting data from tiled");

        let mut path: String = self.address.to_owned();
        path.push_str(endpoint);
        let path = Url::parse(&path)?;

        let response = reqwest::get(path).await?;
        let json = response.json().await?;
        Ok(serde_json::from_value(json)?)
    }
}
impl Client for TiledClient {
    async fn metadata(&self) -> ClientResult<Metadata> {
        self.request::<Metadata>("/api/v1/").await
    }
}
