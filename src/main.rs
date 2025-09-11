use async_graphql::*;

use crate::metadata::Metadata;

mod metadata;

struct TiledClient;

impl TiledClient{
    async fn request(&self) -> reqwest::Response {
        reqwest::get("http://127.0.0.1:8000/api/v1/").await.unwrap()
    }

    async fn get_metadata_struct(&self) -> Metadata {
        let metadata_json = self.request().await.json().await.unwrap();

        let md = serde_json::from_value(metadata_json).unwrap();

        md
    }
}

struct MetadataSchema;

#[Object]
impl MetadataSchema{
    async fn api_version<'ctx>(&self, ctx: &Context<'ctx>) -> i64 {
        let tc = ctx.data::<TiledClient>().unwrap();
        let md = tc.get_metadata_struct().await;

        md.api_version
    }

    async fn library_version<'ctx>(&self, ctx: &Context<'ctx>) -> String {
        let tc = ctx.data::<TiledClient>().unwrap();
        let md = tc.get_metadata_struct().await;

        md.library_version
    }

    async fn queries<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<String> {
        let tc = ctx.data::<TiledClient>().unwrap();
        let md = tc.get_metadata_struct().await;

        md.queries
    }
}

#[tokio::main]
async fn main() {

    let tiled_client = TiledClient;

    let schema = Schema::build(MetadataSchema, EmptyMutation, EmptySubscription)
        .data(tiled_client)
        .finish();

    let res = schema.execute("{ apiVersion libraryVersion queries }").await;

    println!("{:?}", res);
}