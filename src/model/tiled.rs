use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

type SearchResponse = Response<Vec<Resource<NodeAttributes, Value, Value>>, PaginationLinks, Value>;

#[derive(Debug, Deserialize, PartialEq)]
struct Error {
    code: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
struct ResponseContent<Data> {
    error: Option<Error>,
    data: Option<Data>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(try_from = "ResponseContent<Data>")]
enum Content<Data> {
    Error(Error),
    Data(Data),
}

impl<Data> TryFrom<ResponseContent<Data>> for Content<Data> {
    type Error = &'static str;

    fn try_from(
        value: ResponseContent<Data>,
    ) -> Result<Self, <Self as TryFrom<ResponseContent<Data>>>::Error> {
        match value {
            ResponseContent {
                error: None,
                data: None,
            } => Err("Neither data nor error are present"),
            ResponseContent {
                error: None,
                data: Some(data),
            } => Ok(Self::Data(data)),
            ResponseContent {
                error: Some(error),
                data: None,
            } => Ok(Self::Error(error)),
            _ => Err("Both data and error are present"),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Response<Data, Links, Meta> {
    #[serde(flatten)]
    content: Content<Data>,
    links: Option<Links>,
    meta: Option<Meta>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PaginationLinks {
    #[serde(rename = "self")]
    this: String,
    first: String,
    prev: Option<String>,
    next: Option<String>,
    last: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SelfLinkOnly {
    #[serde(rename = "self")]
    this: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ContainerLinks {
    #[serde(rename = "self")]
    this: String,
    search: String,
    full: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ArrayLinks {
    #[serde(rename = "self")]
    this: String,
    full: String,
    block: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct AwkwardLinks {
    #[serde(rename = "self")]
    this: String,
    buffers: String,
    full: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct DataFrameLinks {
    #[serde(rename = "self")]
    this: String,
    full: String,
    partition: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SparseLinks {
    #[serde(rename = "self")]
    this: String,
    full: String,
    block: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum EntryFields {
    Metadata,
    StructureFamily,
    Structure,
    Count,
    Sorting,
    Specs,
    DataSources,
    None,
    AccessBlob,
}

#[derive(Debug, Deserialize, PartialEq)]
struct NodeStructure {
    contents: HashMap<String, Value>,
    count: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Spec {
    name: String,
    version: Option<String>,
}

type Specs = Vec<Spec>;

#[derive(Debug, Deserialize, PartialEq)]
struct Asset {
    data_uri: String,
    is_directory: bool,
    parameter: Option<String>,
    num: Option<i32>,
    id: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Revision {
    revision_number: i32,
    metadata: HashMap<String, Value>,
    specs: Specs,
    access_blob: HashMap<String, Value>,
    time_updates: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, PartialEq)]
struct DataSource {
    id: Option<i32>,
    #[serde(flatten)]
    structure: Option<Structure>,
    mimetype: Option<String>,
    parameters: HashMap<String, Value>,
    assets: Vec<Asset>,
    management: Management,
}

#[derive(Debug, Deserialize, PartialEq)]
struct NodeAttributes {
    ancestors: Vec<String>,
    #[serde(flatten)]
    structure: Option<Structure>,
    specs: Option<Specs>,
    metadata: Option<HashMap<String, Value>>,
    access_blob: Option<HashMap<String, Value>>,
    sorting: Option<Vec<SortingItem>>,
    data_sources: Option<Vec<DataSource>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "structure_family", content = "structure")]
enum Structure {
    Array(ArrayStructure),
    Awkward(AwkwardStructure),
    // Container(ContainerStructure),
    // TODO: What is going on here?
    Container(NodeStructure),
    Sparse(SparseStructure),
    Table(TableStructure),
}

#[derive(Debug, Deserialize, PartialEq)]
struct ArrayStructure {
    data_type: DataType,
    chunks: Vec<Vec<i32>>,
    shape: Vec<i32>,
    dims: Option<Vec<String>>,
    resizable: Union<bool, Vec<bool>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct AwkwardStructure {
    length: i32,
    form: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SparseStructure {
    chunks: Vec<Vec<i32>>,
    shape: Vec<i32>,
    data_type: Option<DataType>,
    coord_data_type: Option<Value>,
    dims: Option<Vec<String>>,
    resizable: Union<bool, Vec<bool>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TableStructure {
    arrow_schema: String,
    npartitions: i32,
    columns: Vec<i32>,
    resizable: Union<bool, Vec<bool>>,
}

#[derive(Debug, Deserialize, PartialEq)]
enum DataType {
    Builtin(BuiltinDataType),
    Struct(StructDataType),
}

#[derive(Debug, Deserialize, PartialEq)]
struct BuiltinDataType {
    endianness: Endianness,
    kind: Kind,
    itemsize: i32,
    dt_units: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StructDataType {
    itemsize: i32,
    fields: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum Endianness {
    Big,
    Little,
    NotApplicable,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case", try_from = "char")]
enum Kind {
    BitField,
    Boolean,
    Integer,
    UnsignedInteger,
    FloatingPoint,
    ComplexFloatingPoint,
    Timedelta,
    Datetime,
    String,
    Unicode,
    Other,
}

impl TryFrom<char> for Kind {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            't' => Self::BitField,
            'b' => Self::Boolean,
            'i' => Self::Integer,
            'u' => Self::UnsignedInteger,
            'f' => Self::FloatingPoint,
            'c' => Self::ComplexFloatingPoint,
            'm' => Self::Timedelta,
            'M' => Self::Datetime,
            'S' => Self::String,
            'U' => Self::Unicode,
            'V' => Self::Other,
            _ => return Err(format!("Unrecognised kind: {value}")),
        })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum Union<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug, Deserialize, PartialEq)]
struct ContainerMeta {
    count: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Resource<Att, Links, Meta> {
    id: String,
    attributes: Att,
    links: Option<Links>,
    meta: Option<Meta>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Management {
    External,
    Immutable,
    Locked,
    Writable,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
enum SortingDirection {
    Ascending,
    Descending,
}

impl SortingDirection {
    fn parse_direction<'de, D: Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
        match Union::<i32, String>::deserialize(des)? {
            Union::Left(1) => Ok(Self::Ascending),
            Union::Left(-1) => Ok(Self::Descending),
            Union::Left(v) => Err(serde::de::Error::custom(format!("Invalid value: {v}"))),
            Union::Right(word) => match word.as_str() {
                "ASCENDING" => Ok(Self::Ascending),
                "DESCENDING" => Ok(Self::Descending),
                other => Err(serde::de::Error::custom(format!("Invalid value: {other}"))),
            },
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct SortingItem {
    key: String,
    #[serde(deserialize_with = "SortingDirection::parse_direction")]
    direction: SortingDirection,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    use super::{Content, Response};
    use crate::test_utils::assert_readable_as;

    #[test]
    fn read_error_response() {
        let raw = r#"{ "data": null, "error": {"code": 23, "message": "broken"}}"#;
        let parsed = serde_json::from_str::<super::SearchResponse>(raw).unwrap();
        assert_eq!(
            parsed,
            Response {
                content: Content::Error(super::Error {
                    code: 23,
                    message: "broken".into()
                }),
                links: None,
                meta: None
            }
        )
    }

    // #[test]
    // fn read_metadata_app() {
    //     assert_readable_as::<super::SearchResponse>("resources/metadata_app.json");
    // }

    #[test]
    fn read_root_response() {
        assert_readable_as::<super::SearchResponse>("resources/search_root.json");
    }
    #[test]
    fn read_container_event_stream() {
        assert_readable_as::<ContainerResponse>("resources/container_event_stream.json");
    }

    #[test]
    fn read_container_run() {
        assert_readable_as::<ContainerResponse>("resources/container_run.json");
    }

    #[test]
    fn read_container_snippet() {
        assert_readable_as::<ContainerResponse>("resources/container_snippet.json");
    }

    type MetadataResponse = super::Response<
        super::Resource<super::NodeAttributes, HashMap<String, Value>, HashMap<String, Value>>,
        HashMap<String, Value>,
        HashMap<String, Value>,
    >;

    type ContainerResponse = super::Response<super::Resource<Value, Value, Value>, Value, Value>;

    #[test]
    fn read_metadata_array() {
        assert_readable_as::<MetadataResponse>("resources/metadata_array.json");
    }

    #[test]
    fn read_metadata_event_stream() {
        assert_readable_as::<MetadataResponse>("resources/metadata_event_stream.json");
    }

    #[test]
    fn read_metadata_run() {
        assert_readable_as::<MetadataResponse>("resources/metadata_run.json");
    }

    #[test]
    fn read_metadata_table() {
        assert_readable_as::<MetadataResponse>("resources/metadata_table.json");
    }

    #[test]
    fn read_search_run_container() {
        assert_readable_as::<ContainerResponse>("resources/search_run_container.json");
    }

    #[test]
    fn read_table_full() {
        assert_readable_as::<super::SearchResponse>("resources/table_full.json");
    }
}
