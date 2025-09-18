use crate::schemas::tiled_metadata::Metadata;

pub trait Client {
    fn metadata(&self) -> impl Future<Output = Metadata> + Send;
}