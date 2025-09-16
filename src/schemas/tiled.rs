use async_graphql::{Context, Object};

use crate::Client;
use crate::schemas::metadata::Metadata;

pub(crate) struct TiledSchema<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledSchema<T> {
    async fn metadata<'ctx>(&self, ctx: &Context<'ctx>) -> Metadata {
        self.0.get_metadata_struct().await
    }
}
