use crate::{clients::client::Client, schemas::tiled_metadata::Metadata};

pub struct TiledClient;

impl TiledClient {
    async fn request(&self) -> reqwest::Response {
        println!("Requesting data from tiled");
        reqwest::get("http://127.0.0.1:8000/api/v1/").await.expect(&format!("Request failed"))
    }
}
impl Client for TiledClient {
    async fn metadata(&self) -> Metadata {
        let metadata_json = self.request().await.json().await.unwrap_or(serde_json::json!(""));

        let md: Metadata = serde_json::from_value(metadata_json).expect("Could not construct metadata struct");

        md
    }
}
