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
