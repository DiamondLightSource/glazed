use std::path::Path;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GlazedConfig {
    pub bind_address: String,

    pub tiled_client: TiledClientConfig,
    pub mock_tiled_client: MockTiledClientConfig,

}
impl GlazedConfig {
    fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        
        config.try_deserialize::<Self>()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TiledClientConfig {
    pub address: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MockTiledClientConfig {
    pub dir_path: String,
}