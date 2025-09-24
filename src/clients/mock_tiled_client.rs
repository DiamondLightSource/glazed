use std::fs::File;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::clients::{Client, ClientResult};
use crate::schemas::tiled_metadata::Metadata;
pub struct MockTiledClient {
    pub dir_path: String,
}

impl MockTiledClient {
    async fn load_file_into_struct<T: DeserializeOwned>(&self, file: &str) -> ClientResult<T> {
        println!("Requesting data from mock");

        let path_str = self.dir_path.to_string() + file;
        let path = Path::new(&path_str);
        let file = File::open(&path)?;

        Ok(serde_json::from_reader(file)?)

    }
}
impl Client for MockTiledClient {
    async fn metadata(&self) -> ClientResult<Metadata> {
        self.load_file_into_struct::<Metadata>(&"tiled_metadata.json").await
    }
}
