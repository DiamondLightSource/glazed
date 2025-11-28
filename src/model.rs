pub(crate) mod app;
pub(crate) mod array;
pub(crate) mod container;
pub(crate) mod event_stream;
pub(crate) mod node;
pub(crate) mod run;
pub(crate) mod table;

use std::collections::HashMap;

use async_graphql::{Context, Object, Result, SimpleObject, Union};
use serde_json::Value;
use tracing::{info, instrument};

use crate::clients::TiledClient;

pub(crate) struct TiledQuery;

#[Object]
impl TiledQuery {
    #[instrument(skip(self, ctx))]
    async fn app_metadata(&self, ctx: &Context<'_>) -> Result<app::AppMetadata> {
        Ok(ctx.data::<TiledClient>()?.app_metadata().await?)
    }

    async fn instrument_session(&self, _ctx: &Context<'_>, name: String) -> InstrumentSession {
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

#[derive(Union)]
enum RunData<'run> {
    Array(ArrayData<'run>),
    Internal(TableData),
}

struct ArrayData<'run> {
    run: &'run Run,
    id: String,
    stream: String,
    attrs: node::Attributes<HashMap<String, Value>, array::ArrayStructure>,
}

#[Object]
impl ArrayData<'_> {
    async fn name(&self) -> &str {
        &self.id
    }
    async fn file(&self) -> Vec<&str> {
        self.attrs
            .data_sources
            .as_deref()
            .unwrap_or_default()
            .iter()
            .flat_map(|det| det.assets.iter().map(|ass| ass.data_uri.as_str()))
            .collect()
        // file: det.assets[0].data_uri.clone(),
    }
    async fn download(&self) -> Vec<String> {
        self.attrs
            .data_sources
            .as_deref()
            .unwrap_or_default()
            .iter()
            .flat_map(|det| det.assets.iter().map(|ass| ass.id))
            .flatten()
            .map(|id| {
                format!(
                    "http://localhost:3000/asset/{}/{}/{}/{}",
                    self.run.data.id, self.stream, self.id, id
                )
            })
            .collect()
    }
}

struct TableData {
    id: String,
    attrs: node::Attributes<HashMap<String, Value>, table::TableStructure>,
}

#[Object]
impl TableData {
    async fn name(&self) -> &str {
        &self.id
    }
    async fn columns(&self) -> &[String] {
        &self.attrs.structure.columns
    }
    async fn data(&self, columns: Vec<String>) -> HashMap<String, Value> {
        columns
            .into_iter()
            .map(|c| (c, Value::Array(vec![])))
            .collect()
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
    async fn data(&self, ctx: &Context<'_>) -> Result<Vec<RunData>> {
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
            for dataset in stream_data.data {
                match dataset.attributes {
                    node::NodeAttributes::Array(attrs) => sources.push(RunData::Array(ArrayData {
                        run: self,
                        stream: stream.id.clone(),
                        id: dataset.id,
                        attrs,
                    })),
                    node::NodeAttributes::Table(attrs) => {
                        sources.push(RunData::Internal(TableData {
                            id: dataset.id,
                            attrs,
                        }))
                    }
                    node::NodeAttributes::Container(cont) => {
                        todo!()
                    }
                }
            }
        }
        Ok(sources)
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
                    // info!("We have an array: {:?}", arr);
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
                    info!("We have something else: {:?}", stream.attributes);
                }
            }
        }

        Ok(sources)
    }
    async fn internal(&self, ctx: &Context<'_>) -> Result<Vec<table::Table>> {
        let client = ctx.data::<TiledClient>()?;
        let run_data = client
            .search::<node::Root>(&self.data.id, &[("include_data_sources", "true")])
            .await?;

        let mut tables: Vec<table::Table> = Vec::new();
        for stream in run_data.data {
            let stream_data = client
                .search::<node::Root>(
                    &format!("{}/{}", self.data.id, stream.id),
                    &[("include_data_sources", "true")],
                )
                .await?;
            dbg!(&stream_data);
            for dataset in stream_data.data {
                if let node::NodeAttributes::Table(table) = dataset.attributes {
                    info!("We have an table: {:?}", table);
                    let p = table
                        .ancestors
                        .into_iter()
                        .chain(vec![dataset.id])
                        .collect::<Vec<String>>()
                        .join("/");
                    info!("path: {:?}", p);

                    let table_data = client.table_full(&p).await?;
                    tables.push(table_data);
                } else {
                    info!("We have something else: {:?}", stream.attributes);
                }
            }
        }
        Ok(tables)
    }
}

#[derive(SimpleObject)]
struct DetectorData {
    name: String,
    file: String,
    download: String,
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema, value};
    use httpmock::MockServer;

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
    async fn instrument_session() {
        todo!();
    }
}
