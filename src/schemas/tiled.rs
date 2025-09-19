use async_graphql::Object;

use crate::schemas::tiled_metadata::Metadata;
use crate::clients::client::{Client, RequestError};

pub(crate) struct TiledSchema<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledSchema<T> {
    async fn metadata(&self) -> Result<Metadata, RequestError> {
        self.0.metadata().await
    }
}
