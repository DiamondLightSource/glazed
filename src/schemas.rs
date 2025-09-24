pub(crate) mod tiled_metadata;

use async_graphql::Object;

use crate::clients::client::{Client, ClientError};

pub(crate) struct TiledSchema<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledSchema<T> {
    async fn metadata(&self) -> async_graphql::Result<tiled_metadata::Metadata, ClientError> {
        self.0.metadata().await
    }
}
