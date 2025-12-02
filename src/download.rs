use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use serde_json::{Value, json};
use tracing::{error, info};

use crate::clients::TiledClient;

const FORWARDED_HEADERS: [&str; 4] = [
    "content-disposition",
    "content-type",
    "content-length",
    "last-modified",
];

pub async fn download(
    State(client): State<TiledClient>,
    Path((run, stream, det, id)): Path<(String, String, String, u32)>,
) -> (StatusCode, HeaderMap, Body) {
    info!("Downloading {run}/{stream}/{det}/{id}");
    let req = client.download(run, stream, det, id).await;
    forward_download_response(req).await
}

async fn forward_download_response(
    response: Result<reqwest::Response, reqwest::Error>,
) -> (StatusCode, HeaderMap, Body) {
    match response {
        Ok(mut resp) => match resp.status().as_u16() {
                200..300  => {
                    let mut headers = HeaderMap::new();
                    for key in FORWARDED_HEADERS {
                        if let Some(value) = resp.headers_mut().remove(key) {
                            headers.insert(key, value);
                        }
                    }
                    let stream = Body::from_stream(resp.bytes_stream());
                    (StatusCode::OK, headers, stream)
                },
                400..500 => (
                    // Probably permission error or non-existent file - forward error to client
                    resp.status(),
                    HeaderMap::new(),
                    Body::from_stream(resp.bytes_stream())
                ),
                100..200 | // ??? check tiled?
                300..400 | // should have followed a redirect
                0..100 | (600..) |  // who needs standards anyway
                500..600 => {
                    let status = resp.status().as_u16();
                    let content = resp.text().await.unwrap_or_else(|e| format!("Unable to read error response: {e}"));
                    (
                        // Whatever we got back, it wasn't what we expected so blame it on tiled
                        StatusCode::SERVICE_UNAVAILABLE,
                        HeaderMap::new(),
                        json!({
                            "detail": "Unexpected response from tiled",
                            "status": status,
                            // Try to parse response as json before giving up and passing a string
                            "response": serde_json::from_str(&content)
                                .unwrap_or(Value::String(content))
                        }).to_string().into()
                    )
                }
            },
        Err(err) => {
            error!("Error sending request to tiled: {err}");
            let (status, message) = if err.is_connect() {
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Could not connect to tiled",
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error making request to tiled",
                )
            };

            (status, HeaderMap::new(), message.into())
        }
    }
}
