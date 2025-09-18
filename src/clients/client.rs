use crate::schemas::tiled_metadata::Metadata;

pub trait Client {
    fn get_metadata_struct(&self) -> impl Future<Output = Metadata> + Send;
}