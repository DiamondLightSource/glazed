use std::fs::File;
use std::path::PathBuf;

use serde::de::DeserializeOwned;

use crate::clients::{Client, ClientResult};
use crate::model::metadata::Metadata;
pub struct MockTiledClient {
    pub dir_path: PathBuf,
}

impl MockTiledClient {
    async fn deserialize_from_file<T: DeserializeOwned>(&self, filename: &str) -> ClientResult<T> {
        println!("Requesting data from mock");

        let path = self.dir_path.join(filename);
        let file = File::open(&path)?;

        Ok(serde_json::from_reader(file)?)
    }
}
impl Client for MockTiledClient {
    async fn metadata(&self) -> ClientResult<Metadata> {
        self.deserialize_from_file("tiled_metadata.json").await
    }
}
