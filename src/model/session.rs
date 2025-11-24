use async_graphql::SimpleObject;
use uuid::Uuid;

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
pub struct Run {
    id: Uuid,
    detectors: Vec<Detector>,
}

impl From<FilterData> for Run {
    fn from(value: FilterData) -> Self {
        Run {
            id: value.id,
            detectors: value
                .attributes
                .metadata
                .start
                .detectors
                .into_iter()
                .map(|name| Detector { name })
                .collect(),
        }
    }
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
