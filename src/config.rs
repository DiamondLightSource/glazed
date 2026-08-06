use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

use config::{Config, ConfigError, File};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct GlazedConfig {
    pub bind_address: SocketAddr,
    pub public_address: Option<Url>,
    pub tiled_client: TiledClientConfig,
    #[serde(default)]
    pub log_level: LogLevel,
}
impl GlazedConfig {
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let config = Config::builder().add_source(File::from(path)).build()?;
        config.try_deserialize()
    }

    pub fn default() -> Self {
        GlazedConfig {
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000),
            public_address: None,
            tiled_client: TiledClientConfig {
                address: Url::parse("http://localhost:8000").expect("Static URL is valid"),
            },
            log_level: LogLevel::Info,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TiledClientConfig {
    pub address: Url,
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
pub enum LogLevel {
    #[default]
    #[serde(alias = "info")]
    Info,
    #[serde(alias = "debug")]
    Debug,
    #[serde(alias = "trace")]
    Trace,
}

impl From<LogLevel> for tracing::level_filters::LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing::level_filters::LevelFilter;

    use crate::config::LogLevel;

    #[test]
    fn level_conversion() {
        assert_eq!(LevelFilter::from(LogLevel::Info), LevelFilter::INFO);
        assert_eq!(LevelFilter::from(LogLevel::Debug), LevelFilter::DEBUG);
        assert_eq!(LevelFilter::from(LogLevel::Trace), LevelFilter::TRACE);
    }
}
