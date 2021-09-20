use async_graphql::{Object, Context, Result, Error};
use mongodb::bson::oid::ObjectId;

use crate::database::DatabaseContext;
use crate::models::Todo;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn add_todo(&self, context: &Context<'_>, content: String) -> Result<String> {
		let context = context.data::<DatabaseContext>()?;
		context.add_todo(Todo::new(content)).await
			.map_err(Error::from)
    }

	pub async fn toggle_done(&self, context: &Context<'_>, id: String) -> Result<u64> {
		let context = context.data::<DatabaseContext>()?;
		context.toggle_done(ObjectId::parse_str(&id)?).await
			.map_err(Error::from)
	}

	pub async fn set_priority(&self, context: &Context<'_>, id: String, new_priority: u64) -> Result<u64> {
		let context = context.data::<DatabaseContext>()?;
		context.set_priority(ObjectId::parse_str(&id)?, new_priority).await
			.map_err(Error::from)
	}

    pub async fn delete_todo(&self, context: &Context<'_>, id: String) -> Result<u64> {
		let context = context.data::<DatabaseContext>()?;
		context.delete_todo(ObjectId::parse_str(&id)?).await
			.map_err(Error::from)
    }
}