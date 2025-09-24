pub(crate) mod tiled_metadata;

use async_graphql::Object;

use crate::clients::{Client, ClientError};

pub(crate) struct TiledQuery<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledQuery<T> {
    async fn metadata(&self) -> async_graphql::Result<tiled_metadata::Metadata, ClientError> {
        self.0.metadata().await
    }
}
