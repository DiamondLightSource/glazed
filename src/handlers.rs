use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use axum::response::{Html, IntoResponse};

use crate::clients::Client;
use crate::model::TiledQuery;

pub async fn graphql_handler<T: Client + Send + Sync + 'static>(
    schema: Extension<Schema<TiledQuery, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let query = req.into_inner().query;

    schema.execute(query).await.into()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use httpmock::MockServer;
    use url::Url;

    use crate::TiledQuery;
    use crate::clients::TiledClient;

    #[tokio::test]
    async fn test_api_version_query() {
        let mock_server = MockServer::start();

        let mock = mock_server
            .mock_async(|when, then| {
                when.method("GET").path("/api/v1/");
                then.status(200)
                    .body_from_file("resources/tiled_metadata.json");
            })
            .await;

        let schema = Schema::build(
            TiledQuery(TiledClient {
                address: Url::parse(&mock_server.base_url()).unwrap(),
            }),
            EmptyMutation,
            EmptySubscription,
        )
        .finish();

        let response = schema.execute("{metadata { apiVersion } }").await;

        assert_eq!(response.data.to_string(), "{metadata: {apiVersion: 0}}");
        mock.assert();
    }
}
