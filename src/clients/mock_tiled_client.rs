use std::fs::File;
use std::path::Path;

use crate::{clients::client::Client, schemas::tiled_metadata::Metadata};

pub struct MockTiledClient;

impl Client for MockTiledClient {
    async fn get_metadata_struct(&self) -> Metadata {
        println!("Requesting data from mock");

        let path = Path::new("./src/metadata.json");
        let file = File::open(&path).expect(&format!("File not found {path:?}"));

        serde_json::from_reader(file).expect(&format!("Failed to deserealize {path:?}"))
    }
}
