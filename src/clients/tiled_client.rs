use reqwest::Url;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::clients::{Client, ClientResult};
use crate::model::app_metadata::AppMetadata;
use crate::model::metadata::Root;

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
    async fn app_metadata(&self) -> ClientResult<AppMetadata> {
        self.request::<AppMetadata>("/api/v1/").await
    }

    async fn run_metadata(&self, id: Uuid) -> ClientResult<Root> {
        self.request::<Root>(&format!("/api/v1/metadata/{id}"))
            .await
    }
}
