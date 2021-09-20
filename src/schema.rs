mod query;
mod mutation;

use async_graphql::{EmptySubscription};
use anyhow::Result;

use query::Query;
use mutation::Mutation;
use crate::database::DatabaseContext;

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub async fn new() -> Result<Schema> {
    let schema = 
        async_graphql::Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(DatabaseContext::new().await?)
            .finish();

    Ok(schema)
}