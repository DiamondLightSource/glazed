use config::{Config, ConfigError, File};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct GlazedConfig {
    pub bind_address: String,

    pub tiled_client: TiledClientConfig,
}
impl GlazedConfig {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        config.try_deserialize::<Self>()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TiledClientConfig {
    pub address: Url,
}
