mod database;
mod models;
mod schema;

use anyhow::Result;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{AddExtensionLayer, Router, Server, extract::Extension, handler::get, response, response::IntoResponse};

async fn graphql_handler(schema: Extension<schema::Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await
        .into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> Result<()> {
    let schema = schema::new().await?;

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    Server::bind(&"0.0.0.0:80".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
