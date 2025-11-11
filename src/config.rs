use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

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
        let config = Config::builder().add_source(File::from(path)).build()?;
        config.try_deserialize()
    }

    pub fn default() -> Self {
        GlazedConfig {
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000),
            tiled_client: TiledClientConfig {
                address: Url::parse("http://localhost:8000").expect("Static URL is valid"),
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TiledClientConfig {
    pub address: Url,
}
