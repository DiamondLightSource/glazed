use crate::{clients::client::Client, schemas::tiled::TiledSchema};
use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

pub async fn graphql_handler<T: Client + Send + Sync + 'static>(
    req: GraphQLRequest,
) -> GraphQLResponse {
    let client = T::new();
    let schema = Schema::build(TiledSchema(client), EmptyMutation, EmptySubscription).finish();

    let query = req.into_inner().query;

    schema.execute(query).await.into()
}

#[cfg(test)]
mod tests {
    use axum::test_helpers::TestClient;

    use crate::handlers::graphql::graphql_handler;

    #[tokio::test]
    async fn test_query() {
        let app = axum::Router::new().route(
            "/graphql",
            axum::routing::post(
                graphql_handler::<crate::clients::mock_tiled_client::MockTiledClient>,
            ),
        );
        let client = TestClient::new(app);

        let json: serde_json::Value =
            serde_json::from_str("{ \"query\": \"{metadata { apiVersion } }\" }").unwrap();

        let x = client.post("/graphql").json(&json).await;

        assert!(x.text().await == "{\"data\":{\"metadata\":{\"apiVersion\":0}}}")
    }
}
