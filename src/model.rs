pub(crate) mod app;
pub(crate) mod array;
pub(crate) mod container;
pub(crate) mod event_stream;
pub(crate) mod filter;
pub(crate) mod node;
pub(crate) mod run;
pub(crate) mod session;
pub(crate) mod table;

use async_graphql::{Context, Object, Result};
use itertools::Itertools;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::clients::TiledClient;

pub(crate) struct TiledQuery;

#[Object]
impl TiledQuery {
    #[instrument(skip(self, ctx))]
    async fn app_metadata(&self, ctx: &Context<'_>) -> Result<app::AppMetadata> {
        Ok(ctx.data::<TiledClient>()?.app_metadata().await?)
    }
    #[instrument(skip(self, ctx))]
    async fn run_metadata(&self, ctx: &Context<'_>, id: Uuid) -> Result<run::RunMetadataRoot> {
        Ok(ctx.data::<TiledClient>()?.run_metadata(id).await?)
    }
    #[instrument(skip(self, ctx))]
    async fn event_stream_metadata(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        stream: String,
    ) -> Result<event_stream::EventStreamMetadataRoot> {
        Ok(ctx
            .data::<TiledClient>()?
            .event_stream_metadata(id, stream)
            .await?)
    }
    #[instrument(skip(self, ctx))]
    async fn array_metadata(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        stream: String,
        array: String,
    ) -> Result<array::ArrayMetadataRoot> {
        Ok(ctx
            .data::<TiledClient>()?
            .array_metadata(id, stream, array)
            .await?)
    }
    #[instrument(skip(self, ctx))]
    async fn table_metadata(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        stream: String,
        table: String,
    ) -> Result<table::TableMetadataRoot> {
        Ok(ctx
            .data::<TiledClient>()?
            .table_metadata(id, stream, table)
            .await?)
    }
    #[instrument(skip(self, ctx))]
    async fn table_full(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        stream: String,
        table: String,
    ) -> Result<table::Table> {
        Ok(ctx
            .data::<TiledClient>()?
            .table_full(id, stream, table)
            .await?)
    }
    #[instrument(skip(self, ctx))]
    async fn search_root(&self, ctx: &Context<'_>) -> Result<run::RunRoot> {
        Ok(ctx.data::<TiledClient>()?.search_root().await?)
    }
    #[instrument(skip(self, ctx))]
    async fn search_run_container(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<event_stream::EventStreamRoot> {
        Ok(ctx.data::<TiledClient>()?.search_run_container(id).await?)
    }
    #[instrument(skip(self, ctx))]
    async fn container_full(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        stream: Option<String>,
    ) -> Result<container::Container> {
        Ok(ctx
            .data::<TiledClient>()?
            .container_full(id, stream)
            .await?)
    }
    #[instrument(skip(self, ctx))]
    async fn instrument(&self, ctx: &Context<'_>, name: String) -> Result<session::Instrument> {
        let instrument = format!(r#""{}""#, name);

        let query_params = &[
            ("fields", "metadata"),
            ("omit_links", "true"),
            ("filter[eq][condition][key]", "start.instrument"),
            ("filter[eq][condition][value]", &instrument),
        ];

        let root = ctx
            .data::<TiledClient>()?
            .query_root(Some(query_params))
            .await;
        info!("root: {root:#?}");

        let runs = root.unwrap();

        let instrument_sessions = runs
            .data
            .into_iter()
            .map(|fd| {
                (
                    fd.attributes.metadata.start.instrument_session.clone(),
                    fd.into(),
                )
            })
            .into_group_map()
            .into_iter()
            .map(|(id, runs)| session::InstrumentSession { id, runs })
            .collect();

        let inst = session::Instrument {
            name,
            instrument_sessions,
        };
        Ok(inst)
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, value};
    use httpmock::MockServer;
    use uuid::Uuid;

    use crate::TiledQuery;
    use crate::clients::TiledClient;

    fn build_schema(url: &str) -> Schema<TiledQuery, EmptyMutation, EmptySubscription> {
        Schema::build(TiledQuery, EmptyMutation, EmptySubscription)
            .data(TiledClient::new(url.parse().unwrap()))
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
    async fn array_metadata() {
        let id = Uuid::parse_str("4866611f-e6d9-4517-bedf-fc5526df57ad").unwrap();
        let stream = "primary";
        let array = "det";
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path(format!("/api/v1/metadata/{id}/{stream}/{array}"));
                then.status(200)
                    .body_from_file("resources/metadata_array.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let query = r#"{ arrayMetadata(id:"4866611f-e6d9-4517-bedf-fc5526df57ad", stream:"primary", array:"det") {data {id}}}"#;
        let response = schema.execute(query).await;
        let exp = value! ({
            "arrayMetadata": { "data": {"id": "det"}}
        });

        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
    #[tokio::test]
    async fn table_metadata() {
        let id = Uuid::parse_str("4866611f-e6d9-4517-bedf-fc5526df57ad").unwrap();
        let stream = "primary";
        let table = "internal";
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path(format!("/api/v1/metadata/{id}/{stream}/{table}"));
                then.status(200)
                    .body_from_file("resources/metadata_table.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let query = r#"{ tableMetadata(id:"4866611f-e6d9-4517-bedf-fc5526df57ad", stream:"primary", table:"internal") {data {id}}}"#;
        let response = schema.execute(query).await;
        let exp = value! ({
            "tableMetadata": { "data": {"id": "internal"}}
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
    #[tokio::test]
    async fn container_full() {
        let id = Uuid::parse_str("5d8f5c3e-0e00-4c5c-816d-70b4b0f41498").unwrap();
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path(format!("/api/v1/container/full/{id}"))
                    .header("accept", "application/json");
                then.status(200)
                    .body_from_file("resources/container_snippet.json");
            })
            .await;
        let schema = build_schema(&server.base_url());
        let query = r#"{containerFull(id: "5d8f5c3e-0e00-4c5c-816d-70b4b0f41498"){contents}}"#;
        let response = schema.execute(query).await;
        let exp = value! ({
            "containerFull": {
              "contents": {
                "primary": {
                  "contents": {},
                  "metadata": {}
                }
              }
            }
        });
        assert_eq!(response.data, exp);
        assert_eq!(response.errors, &[]);
        mock.assert();
    }
}
