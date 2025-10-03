use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use axum::response::{Html, IntoResponse};

use crate::clients::Client;
use crate::model::TiledQuery;

pub async fn graphql_handler<T: Client + Send + Sync + 'static>(
    schema: Extension<Schema<TiledQuery<T>, EmptyMutation, EmptySubscription>>,
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

    use crate::TiledQuery;
    use crate::clients::mock_tiled_client::MockTiledClient;

    #[tokio::test]
    async fn test_api_version_query() {
        let schema = Schema::build(
            TiledQuery(MockTiledClient {
                dir_path: "./resources".into(),
            }),
            EmptyMutation,
            EmptySubscription,
        )
        .finish();

        let response = schema.execute("{metadata { apiVersion } }").await;

        println!("{:?}", response.data.to_string());

        assert!(response.data.to_string() == "{metadata: {apiVersion: 0}}")
    }
}
