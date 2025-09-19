use async_graphql::Object;

use crate::clients::client::{Client, ClientError};
use crate::schemas::tiled_metadata::Metadata;

pub(crate) struct TiledSchema<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledSchema<T> {
    async fn metadata(&self) -> async_graphql::Result<Metadata, ClientError> {
        self.0.metadata().await
    }
}
