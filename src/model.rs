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
    async fn app_metadata(&self) -> Result<app::AppMetadata, ClientError> {
        self.0.app_metadata().await
    }
    #[instrument(skip(self))]
    async fn run_metadata(&self, id: Uuid) -> Result<run::RunMetadataRoot, ClientError> {
        self.0.run_metadata(id).await
    }
    #[instrument(skip(self))]
    async fn event_stream_metadata(
        &self,
        id: Uuid,
        stream: String,
    ) -> Result<event_stream::EventStreamMetadataRoot, ClientError> {
        self.0.event_stream_metadata(id, stream).await
    }
    #[instrument(skip(self))]
    async fn search_root(&self) -> Result<run::RunRoot, ClientError> {
        self.0.search_root().await
    }
    #[instrument(skip(self))]
    async fn search_run_container(
        &self,
        id: Uuid,
    ) -> Result<event_stream::EventStreamRoot, ClientError> {
        self.0.search_run_container(id).await
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, value};
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
    async fn app_metadata() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200)
                    .body_from_file("resources/metadata_app.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let response = schema.execute("{appMetadata { apiVersion } }").await;

        assert_eq!(response.data, value! {{"appMetadata": {"apiVersion": 0}}});
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
    #[tokio::test]
    async fn run_metadata() {
        let id = Uuid::parse_str("5d8f5c3e-0e00-4c5c-816d-70b4b0f41498").unwrap();
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path(format!("/api/v1/metadata/{id}"));
                then.status(200)
                    .body_from_file("resources/metadata_run.json");
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
    #[tokio::test]
    async fn search_root() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/search/");
                then.status(200)
                    .body_from_file("resources/search_root.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let query = r#"{ searchRoot {data{id}}}"#;
        let response = schema.execute(query).await;
        let exp = value! ({
            "searchRoot": { "data": [
                {"id": "4866611f-e6d9-4517-bedf-fc5526df57ad"},
                {"id": "1e37c0ed-e87e-470d-be18-9d7f62f69127"},
                ]
            }
        });

        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
    #[tokio::test]
    async fn search_run_container() {
        let id = Uuid::parse_str("5d8f5c3e-0e00-4c5c-816d-70b4b0f41498").unwrap();
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path(format!("/api/v1/search/{id}"));
                then.status(200)
                    .body_from_file("resources/search_run_container.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let query =
            r#"{searchRunContainer(id: "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498") {data {id}}}"#;
        let response = schema.execute(query).await;
        let exp = value! ({
            "searchRunContainer": { "data": [{"id": "primary"}]}
        });

        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
}
