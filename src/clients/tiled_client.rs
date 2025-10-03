use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::clients::{Client, ClientResult};
use crate::model::metadata::Metadata;

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
