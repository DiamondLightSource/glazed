use std::marker::PhantomData;

use async_graphql::{Context, Object};

use crate::Client;
use crate::schemas::metadata::Metadata;

pub(crate) struct TiledSchema<T>(PhantomData<T>);

impl<T> Default for TiledSchema<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[Object]
impl<T: Client + Send + Sync + 'static> TiledSchema<T> {
    async fn metadata<'ctx>(&self, ctx: &Context<'ctx>) -> Metadata {
        let tiled_client = ctx.data::<T>().unwrap();
        tiled_client.get_metadata_struct().await
    }
}
