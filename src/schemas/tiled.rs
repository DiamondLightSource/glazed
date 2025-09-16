use async_graphql::{Context, Object};

use crate::{schemas::metadata::Metadata};
use crate::{clients::mock_tiled_client::MockTiledClient};

pub(crate) struct TiledSchema;

#[Object]
impl TiledSchema {
    async fn metadata<'ctx>(&self, ctx: &Context<'ctx>) -> Metadata {
        let tiled_client = ctx.data::<MockTiledClient>().unwrap();
        tiled_client.get_metadata_struct().await
    }
}