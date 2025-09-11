use async_graphql::*;

use crate::metadata::Metadata;

mod metadata;


async fn request() -> reqwest::Response {
    reqwest::get("http://127.0.0.1:8000/api/v1/").await.unwrap()
}

async fn get_metadata_struct() -> Metadata {
    let metadata_json = request().await.json().await.unwrap();

    let md = serde_json::from_value(metadata_json).unwrap();

    md
}

struct MetadataSchema;

#[Object]
impl MetadataSchema{
    async fn api_version(&self) -> i64 {
        let md = get_metadata_struct().await;

        md.api_version
    }

    async fn library_version(&self) -> String {
        let md = get_metadata_struct().await;

        md.library_version
    }

    async fn queries(&self) -> Vec<String> {
        let md = get_metadata_struct().await;

        md.queries
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(MetadataSchema, EmptyMutation, EmptySubscription);

    let res = schema.execute("{ apiVersion libraryVersion queries }").await;

    println!("{:?}", res);
}