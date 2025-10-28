pub(crate) mod metadata;

use async_graphql::Object;

use crate::clients::{Client, ClientError, TiledClient};

pub(crate) struct TiledQuery(pub TiledClient);

#[Object]
impl TiledQuery {
    async fn metadata(&self) -> async_graphql::Result<metadata::Metadata, ClientError> {
        self.0.metadata().await
    }
}
