use crate::schemas::metadata::Metadata;

pub struct MockTiledClient;

impl MockTiledClient{
    pub async fn get_metadata_struct(&self) -> Metadata {
        use std::fs::File;
        use std::path::Path;
        let path = Path::new("./src/metadata.json");
        let file = File::open(&path).unwrap();

        let md = serde_json::from_reader(file).unwrap();

        md
    }
}
