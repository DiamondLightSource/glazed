use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use futures::StreamExt as _;
use futures::stream::{FuturesOrdered, FuturesUnordered};
use tracing::info;
use uuid::Uuid;

use crate::clients::{ClientError, ClientResult, TiledClient};
use crate::model::container;
use crate::model::filter::FilterData;

#[derive(Debug, Eq, SimpleObject, PartialEq)]
pub struct Instrument {
    pub(crate) name: String,
    pub(crate) instrument_sessions: Vec<InstrumentSession>,
}

#[derive(Debug, Eq, SimpleObject, PartialEq)]
pub struct InstrumentSession {
    pub(crate) id: String,
    pub(crate) runs: Vec<Run>,
}

#[derive(Debug, Eq, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct Run {
    id: Uuid,
    #[graphql(skip)]
    streams: Vec<String>, // detectors: Vec<Detector>,
}

#[ComplexObject]
impl Run {
    async fn data(&self, ctx: &Context<'_>) -> Result<Vec<Data>> {
        let client = ctx.data::<TiledClient>()?;
        let streams: Vec<container::Container> = self
            .streams
            .iter()
            .map(|stm| client.container_full(self.id, Some(stm)))
            .collect::<FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<ClientResult<_>>()?;

        println!("{streams:#?}");

        let data = streams
            .into_iter()
            .zip(self.streams.iter())
            .flat_map(|(sd, s)| {
                sd.contents
                    .as_object()
                    .expect("Contents is not an object")
                    .clone()
                    .into_iter()
                    .map(|(key, dat)| Data {
                        run: self.id.clone(),
                        name: key,
                        stream: s.clone(),
                    })
            })
            .collect();

        Ok(data)
    }
}

impl From<FilterData> for Run {
    fn from(value: FilterData) -> Self {
        Run {
            id: value.id,
            streams: value
                .attributes
                .metadata
                .stop
                .map(|stp| stp.num_events.into_keys().collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Eq, SimpleObject, PartialEq)]
struct Data {
    run: Uuid,
    stream: String,
    name: String,
}

#[derive(Debug, Eq, SimpleObject, PartialEq)]
pub struct Detector {
    name: String,
    // data: DataFile,
}

#[derive(Debug, Eq, SimpleObject, PartialEq)]
pub struct DataFile {
    file_location: String,
    // download_link: String,
}
