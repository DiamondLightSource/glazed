pub(crate) mod app_metadata;
pub(crate) mod common;
pub(crate) mod metadata;

use async_graphql::Object;
use uuid::Uuid;

use crate::clients::{Client, ClientError};

pub(crate) struct TiledQuery<T>(pub T);

#[Object]
impl<T: Client + Send + Sync + 'static> TiledQuery<T> {
    async fn app_metadata(&self) -> async_graphql::Result<app_metadata::AppMetadata, ClientError> {
        self.0.app_metadata().await
    }
    async fn run_metadata(&self, id: Uuid) -> async_graphql::Result<metadata::Root, ClientError> {
        self.0.run_metadata(id).await
    }
}
