use async_graphql::*;

use crate::schemas::tiled::TiledSchema;

mod schemas;
mod tiled_client;

#[tokio::main]
async fn main() {

    let tiled_client = tiled_client::TiledClient;

    let schema = Schema::build(TiledSchema, EmptyMutation, EmptySubscription)
        .data(tiled_client)
        .finish();

    let res = schema.execute("{ metadata { apiVersion libraryVersion queries formats { table }  aliases { table {textCsv } } } }").await;

    println!("{:?}", res);
}