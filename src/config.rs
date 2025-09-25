use std::path::Path;
use std::net::SocketAddr;

use config::{Config, ConfigError, File};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct GlazedConfig {
    pub bind_address: SocketAddr,

    pub tiled_client: TiledClientConfig,
}
impl GlazedConfig {
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::from(path))
            .build()?;

        config.try_deserialize()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TiledClientConfig {
    pub address: Url,
}
