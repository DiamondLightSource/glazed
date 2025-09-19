use crate::{clients::client::{Client, ClientError}, schemas::tiled_metadata::Metadata};

pub struct TiledClient;

impl TiledClient {
    async fn request<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T, ClientError> {
        println!("Requesting data from tiled");

        let mut path: String = String::from("http://127.0.0.1:8000");
        path.push_str(endpoint);
        let path = reqwest::Url::parse(&path)?;

        let req = reqwest::get(path).await?;
        let json = req.json().await?;
        // serde_json::from_value(json).map_err(|e| ClientError::Serde(e))
        Ok(serde_json::from_value(json)?)
        // serde_json::from_value(json)?
    }
}
impl Client for TiledClient {
    async fn metadata(&self) -> Result<Metadata, ClientError> {
        self.request::<Metadata>("/api/v1/").await
    }
}
