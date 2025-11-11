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

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, Value, value};
    use httpmock::MockServer;
    use url::Url;
    use uuid::Uuid;

    use crate::TiledQuery;
    use crate::clients::TiledClient;

    fn build_schema(url: &str) -> Schema<TiledQuery, EmptyMutation, EmptySubscription> {
        Schema::build(
            TiledQuery(TiledClient {
                address: Url::parse(url).unwrap(),
            }),
            EmptyMutation,
            EmptySubscription,
        )
        .finish()
    }

    #[tokio::test]
    async fn test_run_metadata_query() {
        let id = Uuid::parse_str("5d8f5c3e-0e00-4c5c-816d-70b4b0f41498").unwrap();
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path(format!("/api/v1/metadata/{id}"));
                then.status(200)
                    .body_from_file("resources/run_metadata.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let query = r#"{ runMetadata(id: "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498") {data {id}}}"#;
        let response = schema.execute(query).await;
        let exp = value! ({
            "runMetadata": { "data": {"id": "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498"}}
        });

        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
}
