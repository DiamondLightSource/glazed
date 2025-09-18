use async_graphql::Object;

use crate::schemas::tiled_metadata::Metadata;
use crate::clients::client::Client;

pub(crate) struct TiledSchema<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledSchema<T> {
    async fn metadata(&self) -> Metadata {
        self.0.get_metadata_struct().await
    }
}
