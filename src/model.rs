pub(crate) mod app;
pub(crate) mod array;
pub(crate) mod container;
pub(crate) mod event_stream;
pub(crate) mod node;
pub(crate) mod run;
pub(crate) mod table;

use async_graphql::{Context, Object, Result, SimpleObject};
use tracing::{info, instrument};

use crate::clients::TiledClient;

pub(crate) struct TiledQuery;

#[Object]
impl TiledQuery {
    #[instrument(skip(self, ctx))]
    async fn app_metadata(&self, ctx: &Context<'_>) -> Result<app::AppMetadata> {
        Ok(ctx.data::<TiledClient>()?.app_metadata().await?)
    }
    async fn root(&self, ctx: &Context<'_>) -> Result<Root> {
        let root = ctx
            .data::<TiledClient>()?
            .search::<node::Root>("", &[])
            .await
            .unwrap();
        Ok(Root { node: root })
    }

    async fn instrument_session(&self, ctx: &Context<'_>, name: String) -> InstrumentSession {
        InstrumentSession { name }
    }
}

struct InstrumentSession {
    name: String,
}

#[Object]
impl InstrumentSession {
    async fn name(&self) -> &str {
        &self.name
    }
    async fn runs(&self, ctx: &Context<'_>) -> Result<Vec<Run>> {
        let root = ctx
            .data::<TiledClient>()?
            .search::<node::Root>(
                "",
                &[
                    ("filter[eq][condition][key]", "start.instrument_session"),
                    (
                        "filter[eq][condition][value]",
                        &format!(r#""{}""#, self.name),
                    ),
                    ("include_data_sources", "true"),
                ],
            )
            .await?;
        Ok(root.data.into_iter().map(|d| Run { data: d }).collect())
    }
}

struct Run {
    data: node::Data,
}

#[Object]
impl Run {
    async fn id(&self) -> &str {
        &self.data.id
    }
    async fn external(&self, ctx: &Context<'_>) -> Result<Vec<DetectorData>> {
        let client = ctx.data::<TiledClient>()?;
        let run_data = client
            .search::<node::Root>(&self.data.id, &[("include_data_sources", "true")])
            .await?;

        let mut sources = Vec::new();
        for stream in run_data.data {
            let stream_data = client
                .search::<node::Root>(
                    &format!("{}/{}", self.data.id, stream.id),
                    &[("include_data_sources", "true")],
                )
                .await?;
            dbg!(&stream_data);
            for dataset in stream_data.data {
                if let node::NodeAttributes::Array(arr) = dataset.attributes {
                    info!("We have an array: {:?}", arr);
                    let det = arr.data_sources.expect("No datasources")[0].clone();
                    let det_dat = DetectorData {
                        file: det.assets[0].data_uri.clone(),
                        download: format!(
                            "http://localhost:3000/asset/{}/{}/{}/{}",
                            self.data.id,
                            stream.id.clone(),
                            dataset.id,
                            det.assets[0].id.unwrap()
                        ),
                        name: dataset.id.clone(),
                    };
                    sources.push(det_dat);
                } else {
                    info!("We have: {:?}", stream.attributes);
                }
            }
        }

        Ok(sources)
    }
}

#[derive(SimpleObject)]
struct DetectorData {
    name: String,
    file: String,
    download: String,
}

struct Root {
    node: node::Root,
}

#[Object]
impl Root {
    async fn data(&self, ctx: &Context<'_>) -> Vec<Data> {
        self.node
            .data
            .iter()
            .cloned()
            .map(|d| Data { contents: d })
            .collect()
    }
}

struct Data {
    contents: node::Data,
}

#[Object]
impl Data {
    async fn id(&self) -> &str {
        &self.contents.id
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
