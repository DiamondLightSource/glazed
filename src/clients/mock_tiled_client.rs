use std::fs::File;
use std::path::Path;

use crate::{
    clients::client::{Client, ClientResult},
    schemas::tiled_metadata::Metadata,
};
pub struct MockTiledClient;

impl Client for MockTiledClient {
    fn new() -> Self {
        return MockTiledClient;
    }

    async fn metadata(&self) -> ClientResult<Metadata> {
        println!("Requesting data from mock");

        let path = Path::new("./src/metadata.json");
        let file = File::open(&path)?;

        Ok(serde_json::from_reader(file)?)
    }
}
