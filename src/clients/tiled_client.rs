use crate::{clients::client::{Client, RequestError}, schemas::tiled_metadata::Metadata};

pub struct TiledClient;

impl TiledClient {
    async fn request<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T, RequestError> {
        println!("Requesting data from tiled");

        let mut path: String = String::from("http://127.0.0.1:8000");
        path.push_str(endpoint);
        let path = reqwest::Url::parse(&path).map_err(|e| RequestError::Parse(e))?;

        let req = reqwest::get(path).await.map_err(|e| RequestError::Reqwest(e))?;
        let json = req.json().await.map_err(|e| RequestError::Reqwest(e))?;
        serde_json::from_value(json).map_err(|e| RequestError::Serde(e))
    }
}
impl Client for TiledClient {
    async fn metadata(&self) -> Result<Metadata, RequestError> {
        self.request::<Metadata>("/api/v1/").await
    }
}
