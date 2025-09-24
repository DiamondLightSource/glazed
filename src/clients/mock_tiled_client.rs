use std::fs::File;
use std::path::Path;

use crate::clients::{Client, ClientResult};
use crate::schemas::tiled_metadata::Metadata;
pub struct MockTiledClient;

impl Client for MockTiledClient {
    async fn metadata(&self) -> ClientResult<Metadata> {
        println!("Requesting data from mock");

        let path = Path::new("./src/resources/tiled_metadata.json");
        let file = File::open(&path)?;

        Ok(serde_json::from_reader(file)?)
    }
}
