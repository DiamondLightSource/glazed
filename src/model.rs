pub(crate) mod app_metadata;

use async_graphql::Object;

use crate::clients::{Client, ClientError};

pub(crate) struct TiledQuery<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledQuery<T> {
    async fn app_metadata(&self) -> async_graphql::Result<app_metadata::AppMetadata, ClientError> {
        self.0.app_metadata().await
    }
}
