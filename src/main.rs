use std::error;
use std::fmt::Arguments;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Extension, Router};

mod cli;
mod clients;
mod config;
mod handlers;
mod model;
#[cfg(test)]
mod test_utils;

use cli::{Cli, Commands};
use serde::Serialize;
use serde::ser::{SerializeSeq, SerializeTuple as _};
use tokio::select;
use tokio::signal::unix::{SignalKind, signal};
use tracing::info;

use crate::clients::TiledClient;
use crate::config::GlazedConfig;
use crate::handlers::{graphiql_handler, graphql_handler};
use crate::model::TiledQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::init();
    let config;

    if let Some(config_filepath) = cli.config_filepath {
        info!("Loading config from {config_filepath:?}");
        config = GlazedConfig::from_file(&config_filepath)?;
        info!("Config loaded");
    } else {
        info!("Using default config");
        config = GlazedConfig::default();
    }
    match cli.command {
        Commands::Serve => serve(config).await,
        Commands::Demo => demo().await,
    }
}

async fn demo() -> Result<(), Box<dyn error::Error>> {
    let client = reqwest::Client::new();

    let query = Query {
        filters: vec![
            Filter::Eq(EqFilter {
                key: "start.instrument".into(),
                value: "adsim".into(),
            }),
            Filter::Eq(EqFilter {
                key: "start.instrument_session".into(),
                value: "cm12345-1".into(),
            }),
        ],
    };
    let req = client
        .get("http://localhost:8407/api/v1/search/")
        .query(&query);
    let resp = req.send().await?;

    println!("{resp:#?}");
    println!("{}", resp.text().await?);
    Ok(())
}

struct Query {
    filters: Vec<Filter>,
}
impl Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for filter in &self.filters {
            filter.serialize_into::<S>(&mut seq)?;
        }
        seq.end()
    }
}

enum Filter {
    Eq(EqFilter),
}

impl Filter {
    fn serialize_into<S>(&self, ser: &mut S::SerializeSeq) -> Result<(), S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Filter::Eq(eq_filter) => eq_filter.serialize_into::<S>(ser),
        }
    }
}

struct EqFilter {
    key: String,
    value: String,
}

impl EqFilter {
    fn serialize_into<S>(&self, ser: &mut S::SerializeSeq) -> Result<(), S::Error>
    where
        S: serde::Serializer,
    {
        ser.serialize_element::<(&str, &str)>(&("filter[eq][condition][key]".into(), &self.key))?;
        ser.serialize_element::<(&str, Arguments)>(&(
            "filter[eq][condition][value]".into(),
            format_args!(r#""{}""#, self.value),
        ))
    }
}

impl Serialize for EqFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(1)?;
        seq.serialize_element(&("filter[eq][condition][key]", &self.key))?;
        seq.serialize_element(&(
            "filter[eq][condition][value]",
            &format!(r#""{}""#, self.value),
        ))?;
        seq.end()
    }
}


async fn serve(config: GlazedConfig) -> Result<(), Box<dyn error::Error>> {
    let schema = Schema::build(
        TiledQuery(TiledClient {
            address: config.tiled_client.address.to_owned(),
        }),
        EmptyMutation,
        EmptySubscription,
    )
    .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql_handler))
        .fallback((
            StatusCode::NOT_FOUND,
            Html(include_str!("../static/404.html")),
        ))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("Serving glazed at {:?}", config.bind_address);

    Ok(axum::serve(listener, app)
        .with_graceful_shutdown(signal_handler())
        .await?)
}

async fn signal_handler() {
    let mut term = signal(SignalKind::terminate()).expect("Failed to create SIGTERM listener");
    let mut int = signal(SignalKind::interrupt()).expect("Failed to create SIGINT listener");
    let mut quit = signal(SignalKind::interrupt()).expect("Failed to create SIGQUIT listener");
    let sig = select! {
         _ = term.recv() => "SIGTERM",
        _ = int.recv() => "SIGINT",
        _ = quit.recv() => "SIGQUIT",
    };
    info!("Server interrupted by {sig}");
}
