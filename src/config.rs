use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

use config::{Config, ConfigError, File};
use serde::{Deserialize, Deserializer};
use url::Url;

const DEFAULT_PORT: u16 = 3000;

#[derive(Deserialize, Debug, Clone)]
pub struct GlazedConfig {
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,
    #[serde(
        default = "default_public_address",
        deserialize_with = "validate_public_address"
    )]
    pub public_address: Url,
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
            bind_address: default_bind_address(),
            public_address: default_public_address(),
            tiled_client: TiledClientConfig {
                address: Url::parse("http://localhost:8000").expect("Static URL is valid"),
            },
            log_level: LogLevel::Info,
        }
    }
    pub fn endpoint(&self, endpoint: &str) -> String {
        let mut addr = self.public_address.clone();
        addr.path_segments_mut()
            .expect("base address can be a base")
            .pop_if_empty()
            .push(endpoint);
        addr.to_string()
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

fn default_bind_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEFAULT_PORT)
}

fn default_public_address() -> Url {
    let mut addr = Url::parse("http://localhost").expect("Static URL is valid");
    addr.set_port(Some(DEFAULT_PORT)).unwrap();
    addr
}

fn validate_public_address<'de, D: Deserializer<'de>>(des: D) -> Result<Url, D::Error> {
    let url = Url::deserialize(des)?;
    if url.cannot_be_a_base() {
        Err(serde::de::Error::custom("URL cannot be a base"))
    } else {
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use tracing::level_filters::LevelFilter;
    use url::Url;

    use crate::config::{GlazedConfig, LogLevel, default_bind_address, default_public_address};

    #[test]
    fn level_conversion() {
        assert_eq!(LevelFilter::from(LogLevel::Info), LevelFilter::INFO);
        assert_eq!(LevelFilter::from(LogLevel::Debug), LevelFilter::DEBUG);
        assert_eq!(LevelFilter::from(LogLevel::Trace), LevelFilter::TRACE);
    }

    #[rstest::rstest]
    #[case::no_trailing_slash("http://example.com", "http://example.com/graphql")]
    #[case::trailing_slash("http://example.com/", "http://example.com/graphql")]
    #[case::non_empty_path("http://example.com/extra", "http://example.com/extra/graphql")]
    #[case::path_and_trailing("http://example.com/extra/", "http://example.com/extra/graphql")]
    fn graphql_endpoint(#[case] base: &str, #[case] complete: &str) {
        let mut config = GlazedConfig::default();
        config.public_address = Url::parse(base).unwrap();
        assert_eq!(config.endpoint("graphql"), complete)
    }

    #[test]
    fn default_addresses() {
        assert_eq!(default_bind_address().to_string(), "127.0.0.1:3000");
        assert!(!default_public_address().cannot_be_a_base());
        assert_eq!(
            default_public_address().to_string(),
            "http://localhost:3000/"
        );
    }
}
