pub(crate) mod app;
pub(crate) mod array;
pub(crate) mod container;
pub(crate) mod event_stream;
pub(crate) mod node;
pub(crate) mod run;
pub(crate) mod table;

use std::collections::HashMap;

use async_graphql::{Context, Object, Result, Union};
use serde_json::Value;
use tracing::{info, instrument};

use crate::clients::TiledClient;
use crate::model::node::NodeAttributes;

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
impl<'run> ArrayData<'run> {
    async fn name(&self) -> &str {
        &self.id
    }
    async fn files<'ad>(&'ad self) -> Vec<Asset<'ad>> {
        self.attrs
            .data_sources
            .as_deref()
            .unwrap_or_default()
            .iter()
            .flat_map(|source| source.assets.iter())
            .map(|a| Asset {
                data: self,
                asset: a,
            })
            .collect()
    }
}

struct Asset<'a> {
    asset: &'a node::Asset,
    data: &'a ArrayData<'a>,
}

#[Object]
impl Asset<'_> {
    async fn file(&self) -> &str {
        &self.asset.data_uri
    }
    async fn download(&self) -> Option<String> {
        let id = self.asset.id?;
        Some(format!(
            "http://localhost:3000/asset/{}/{}/{}/{}",
            self.data.run.data.id, self.data.stream, self.data.id, id
        ))
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
    async fn data(
        &self,
        ctx: &Context<'_>,
        columns: Vec<String>,
    ) -> Result<HashMap<String, Vec<Value>>> {
        let client = ctx.data::<TiledClient>()?;
        let p = self
            .attrs
            .ancestors
            .iter()
            .chain(vec![&self.id])
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("/");
        info!("path: {:?}", p);

        let table_data = client.table_full(&p, columns).await?;
        Ok(table_data)
    }
}

struct Run {
    data: node::Data,
}

#[Object]
impl Run {
    async fn scan_number(&self) -> Option<i64> {
        if let NodeAttributes::Container(attr) = &self.data.attributes {
            attr.metadata.start_doc().map(|sd| sd.scan_id)
        } else {
            None
        }
    }
    async fn id(&self) -> &str {
        &self.data.id
    }
    async fn data(&self, ctx: &Context<'_>) -> Result<Vec<RunData<'_>>> {
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
                    NodeAttributes::Array(attrs) => sources.push(RunData::Array(ArrayData {
                        run: self,
                        stream: stream.id.clone(),
                        id: dataset.id,
                        attrs,
                    })),
                    NodeAttributes::Table(attrs) => sources.push(RunData::Internal(TableData {
                        id: dataset.id,
                        attrs,
                    })),
                    NodeAttributes::Container(_) => {}
                }
            }
        }
        Ok(sources)
    }
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
