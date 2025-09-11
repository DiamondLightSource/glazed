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

        let md: Metadata = serde_json::from_value(metadata_json).unwrap();

        md
    }
}
struct TiledSchema;

#[Object]
impl TiledSchema {
    async fn metadata<'ctx>(&self, ctx: &Context<'ctx>) -> Metadata {
        let tiled_client = ctx.data::<TiledClient>().unwrap();
        tiled_client.get_metadata_struct().await
    }
}


#[tokio::main]
async fn main() {

    let tiled_client = TiledClient;

    // let schema = Metadata{formats : Formats{}};
    // let schema = Metadata;
    let schema = TiledSchema;

    let schema = Schema::build(schema, EmptyMutation, EmptySubscription)
        .data(tiled_client)
        .finish();

    let res = schema.execute("{ metadata { apiVersion libraryVersion queries } }").await;

    println!("{:?}", res);
}