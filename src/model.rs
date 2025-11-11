pub(crate) mod app;
pub(crate) mod container;
pub(crate) mod event_stream;
pub(crate) mod node;
pub(crate) mod run;

use async_graphql::Object;
use tracing::instrument;
use uuid::Uuid;

use crate::clients::{ClientError, TiledClient};

pub(crate) struct TiledQuery(pub TiledClient);

#[Object]
impl TiledQuery {
    #[instrument(skip(self))]
    async fn app_metadata(&self) -> async_graphql::Result<app::AppMetadata, ClientError> {
        self.0.app_metadata().await
    }
    #[instrument(skip(self))]
    async fn run_metadata(
        &self,
        id: Uuid,
    ) -> async_graphql::Result<run::RunMetadataRoot, ClientError> {
        self.0.run_metadata(id).await
    }
    #[instrument(skip(self))]
    async fn event_stream_metadata(
        &self,
        id: Uuid,
        stream: String,
    ) -> async_graphql::Result<event_stream::EventStreamMetadataRoot, ClientError> {
        self.0.event_stream_metadata(id, stream).await
    }
}
