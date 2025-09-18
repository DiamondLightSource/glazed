use crate::{clients::client::Client, schemas::metadata::Metadata};

pub struct MockTiledClient;

impl Client for MockTiledClient {
    async fn get_metadata_struct(&self) -> Metadata {
        println!("Requesting data from mock");
        use std::fs::File;
        use std::path::Path;
        let path = Path::new("./src/metadata.json");
        let file = File::open(&path).unwrap();

        let md = serde_json::from_reader(file).unwrap();

        md
    }
}
