// Auto-generated with JSON to serde tool

use async_graphql::SimpleObject;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "api_version")]
    pub api_version: i64,
    #[serde(rename = "library_version")]
    pub library_version: String,
    pub formats: Formats,
    pub aliases: Aliases,
    pub queries: Vec<String>,
    pub authentication: Authentication,
    pub links: Links,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Formats {
    pub table: Vec<String>,
    pub container: Vec<String>,
    pub array: Vec<String>,
    pub awkward: Vec<String>,
    pub sparse: Vec<String>,
    #[serde(rename = "xarray_dataset")]
    pub xarray_dataset: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Aliases {
    pub table: Table,
    pub container: Container,
    pub array: Array,
    pub awkward: Awkward,
    pub sparse: Sparse,
    #[serde(rename = "xarray_dataset")]
    pub xarray_dataset: XarrayDataset,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    #[serde(rename = "application/vnd.apache.arrow.file")]
    pub application_vnd_apache_arrow_file: Vec<String>,
    #[serde(rename = "text/csv")]
    pub text_csv: Vec<String>,
    #[serde(rename = "application/vnd.ms-excel")]
    pub application_vnd_ms_excel: Vec<String>,
    #[serde(rename = "text/html")]
    pub text_html: Vec<String>,
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    pub application_vnd_openxmlformats_officedocument_spreadsheetml_sheet: Vec<String>,
    #[serde(rename = "application/json")]
    pub application_json: Vec<String>,
    #[serde(rename = "application/x-hdf5")]
    pub application_x_hdf5: Vec<String>,
    #[serde(rename = "application/x-parquet")]
    pub application_x_parquet: Vec<String>,
    #[serde(rename = "application/netcdf")]
    pub application_netcdf: Vec<String>,
    #[serde(rename = "text/plain")]
    pub text_plain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    #[serde(rename = "application/json")]
    pub application_json: Vec<String>,
    #[serde(rename = "application/x-hdf5")]
    pub application_x_hdf5: Vec<String>,
    #[serde(rename = "application/x-parquet")]
    pub application_x_parquet: Vec<String>,
    #[serde(rename = "application/vnd.apache.arrow.file")]
    pub application_vnd_apache_arrow_file: Vec<String>,
    #[serde(rename = "application/netcdf")]
    pub application_netcdf: Vec<String>,
    #[serde(rename = "text/plain")]
    pub text_plain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    #[serde(rename = "application/json")]
    pub application_json: Vec<String>,
    #[serde(rename = "text/csv")]
    pub text_csv: Vec<String>,
    #[serde(rename = "application/vnd.ms-excel")]
    pub application_vnd_ms_excel: Vec<String>,
    #[serde(rename = "image/png")]
    pub image_png: Vec<String>,
    #[serde(rename = "image/tiff")]
    pub image_tiff: Vec<String>,
    #[serde(rename = "text/html")]
    pub text_html: Vec<String>,
    #[serde(rename = "application/x-hdf5")]
    pub application_x_hdf5: Vec<String>,
    #[serde(rename = "application/x-parquet")]
    pub application_x_parquet: Vec<String>,
    #[serde(rename = "application/vnd.apache.arrow.file")]
    pub application_vnd_apache_arrow_file: Vec<String>,
    #[serde(rename = "application/netcdf")]
    pub application_netcdf: Vec<String>,
    #[serde(rename = "text/plain")]
    pub text_plain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Awkward {
    #[serde(rename = "application/zip")]
    pub application_zip: Vec<String>,
    #[serde(rename = "application/json")]
    pub application_json: Vec<String>,
    #[serde(rename = "application/vnd.apache.arrow.file")]
    pub application_vnd_apache_arrow_file: Vec<String>,
    #[serde(rename = "application/x-hdf5")]
    pub application_x_hdf5: Vec<String>,
    #[serde(rename = "application/x-parquet")]
    pub application_x_parquet: Vec<String>,
    #[serde(rename = "application/netcdf")]
    pub application_netcdf: Vec<String>,
    #[serde(rename = "text/plain")]
    pub text_plain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Sparse {
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    pub application_vnd_openxmlformats_officedocument_spreadsheetml_sheet: Vec<String>,
    #[serde(rename = "application/vnd.apache.arrow.file")]
    pub application_vnd_apache_arrow_file: Vec<String>,
    #[serde(rename = "text/csv")]
    pub text_csv: Vec<String>,
    #[serde(rename = "text/html")]
    pub text_html: Vec<String>,
    #[serde(rename = "application/json")]
    pub application_json: Vec<String>,
    #[serde(rename = "application/x-hdf5")]
    pub application_x_hdf5: Vec<String>,
    #[serde(rename = "application/x-parquet")]
    pub application_x_parquet: Vec<String>,
    #[serde(rename = "application/netcdf")]
    pub application_netcdf: Vec<String>,
    #[serde(rename = "text/plain")]
    pub text_plain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct XarrayDataset {
    #[serde(rename = "application/x-netcdf")]
    pub application_x_netcdf: Vec<String>,
    #[serde(rename = "application/vnd.apache.arrow.file")]
    pub application_vnd_apache_arrow_file: Vec<String>,
    #[serde(rename = "text/csv")]
    pub text_csv: Vec<String>,
    #[serde(rename = "text/html")]
    pub text_html: Vec<String>,
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    pub application_vnd_openxmlformats_officedocument_spreadsheetml_sheet: Vec<String>,
    #[serde(rename = "application/json")]
    pub application_json: Vec<String>,
    #[serde(rename = "application/x-hdf5")]
    pub application_x_hdf5: Vec<String>,
    #[serde(rename = "application/x-parquet")]
    pub application_x_parquet: Vec<String>,
    #[serde(rename = "application/netcdf")]
    pub application_netcdf: Vec<String>,
    #[serde(rename = "text/plain")]
    pub text_plain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    pub required: bool,
    pub providers: Vec<Value>,
    pub links: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub documentation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "root_path")]
    pub root_path: String,
}