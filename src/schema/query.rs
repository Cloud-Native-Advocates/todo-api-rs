use async_graphql::{Object, Context, Result};

use crate::{database::DatabaseContext, models::Todo};

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    pub async fn get_todos(&self, context: &Context<'_>) -> Result<Vec<Todo>> {
		let context = context.data::<DatabaseContext>()?;
		Ok(context.get_todos().await?)
    }
}