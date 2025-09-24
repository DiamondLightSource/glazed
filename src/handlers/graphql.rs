use crate::{clients::client::Client, schemas::TiledSchema};
use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;

pub async fn graphql_handler<T: Client + Send + Sync + 'static>(
    schema: Extension<Schema<TiledSchema<T>, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let query = req.into_inner().query;

    schema.execute(query).await.into()
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptySubscription, EmptyMutation, Schema};

    use crate::TiledSchema;
    use crate::clients::mock_tiled_client::MockTiledClient;

    #[tokio::test]
    async fn test_api_version_query() {
        let schema = Schema::build(
            TiledSchema(MockTiledClient),
            EmptyMutation,
            EmptySubscription,
        )
        .finish();

        let response = schema.execute("{metadata { apiVersion } }").await;

        println!("{:?}", response.data.to_string());

        assert!(response.data.to_string() == "{metadata: {apiVersion: 0}}")
    }
}
