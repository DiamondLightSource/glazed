use async_graphql::{Context, Object};

use crate::{schemas::metadata::Metadata};
use crate::{tiled_client::TiledClient};

pub(crate) struct TiledSchema;

#[Object]
impl TiledSchema {
    async fn metadata<'ctx>(&self, ctx: &Context<'ctx>) -> Metadata {
        let tiled_client = ctx.data::<TiledClient>().unwrap();
        tiled_client.get_metadata_struct().await
    }
}