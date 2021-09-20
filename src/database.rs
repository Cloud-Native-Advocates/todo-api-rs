mod credentials;

use anyhow::{Result, Error};
use futures::TryStreamExt;
use mongodb::{Client, Collection, bson::{Bson, doc, oid::ObjectId}};

use crate::models::Todo;
use credentials::Credentials;

pub struct DatabaseContext {
    todos: Collection<Todo>,
}

impl DatabaseContext {
    pub async fn new() -> Result<Self> {
        let credentials = Credentials::from_environment()?;

        let todos = Self::connect(&credentials).await?
            .database(&credentials.db_name)
            .collection::<Todo>("todos");

        Ok(Self { todos })
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        self.todos
            .find(None, None).await?
            .try_collect().await
            .map_err(Error::from)
    }

    pub async fn add_todo(&self, todo: Todo) -> Result<String> {
        self.todos
            .insert_one(todo, None).await
            .map(|insert_one_result| {
                match insert_one_result.inserted_id {
                    Bson::ObjectId(id) => id.to_string(),
                    _ => "".into()
                }
            })
            .map_err(Error::from)
    }

    pub async fn toggle_done(&self, id: ObjectId) -> Result<u64> {
        let filter = doc!{ "_id" : Bson::ObjectId(id) };
        let update = vec![doc!{ "$set" : { "done" : { "$not" : "$done" } } }];
        self.todos.update_one(filter, update, None).await
            .map(|update_result| update_result.modified_count)
            .map_err(Error::from)
    }

    pub async fn set_priority(&self, id: ObjectId, new_priotity: u64) -> Result<u64> {
        let filter = doc!{ "_id" : Bson::ObjectId(id) };
        let update = doc!{ "$set" : { "priority" : new_priotity.to_string() } };
        self.todos.update_one(filter, update, None).await
            .map(|update_result| update_result.modified_count)
            .map_err(Error::from)
    }

    pub async fn delete_todo(&self, id: ObjectId) -> Result<u64> {
        self.todos
            .delete_one(doc!{ "_id" : Bson::ObjectId(id) }, None).await
            .map(|delete_result| delete_result.deleted_count)
            .map_err(Error::from)
    }

    async fn connect(Credentials { user, password, host, .. }: &Credentials) -> Result<Client> {
        let connection_string = format!("mongodb+srv://{}:{}@{}", user, password, host);
        Client::with_uri_str(connection_string).await
            .map_err(Error::from)
    }
}
