use crate::{clients::client::Client, schemas::metadata::Metadata};

pub struct TiledClient;

impl TiledClient {
    async fn request(&self) -> reqwest::Response {
        println!("Requesting data from tiled");
        reqwest::get("http://127.0.0.1:8000/api/v1/").await.unwrap()
    }
}
impl Client for TiledClient {
    async fn get_metadata_struct(&self) -> Metadata {
        let metadata_json = self.request().await.json().await.unwrap();

        let md: Metadata = serde_json::from_value(metadata_json).unwrap();

        md
    }
}
