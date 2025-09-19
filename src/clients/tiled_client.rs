use crate::{clients::client::Client, schemas::tiled_metadata::Metadata};

pub struct TiledClient;

impl TiledClient {
    async fn request<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> T {
        println!("Requesting data from tiled");

        let mut path: String = String::from("http://127.0.0.1:8000");
        path.push_str(endpoint);
        let path = reqwest::Url::parse(&path).unwrap();

        let req = reqwest::get(path).await.expect(&format!("Request failed"));
        let json = req.json().await.unwrap();
        serde_json::from_value(json).unwrap()
    }
}
impl Client for TiledClient {
    async fn metadata(&self) -> Metadata {
        self.request::<Metadata>("/api/v1/").await

    }
}
