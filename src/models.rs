use serde::{Deserialize, Serialize};
use async_graphql::{SimpleObject, ComplexObject};

#[derive(Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[graphql(skip)]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub content: String,
    pub done: bool,
    pub priority: u64,
}

impl Todo {
    pub fn new<T: Into<String>>(content: T) -> Self {
        Self {
            id: None,
            content: content.into(),
            done: false,
            priority: 0,
        }
    }
}

#[ComplexObject]
impl Todo {
    #[graphql(name = "id")]
    async fn id_(&self) -> String {
        if let Some(id) = self.id {
            format!("{}", id)
        } else {
            "".into()
        }
    }

    #[graphql(name = "content")]
    async fn content_(&self) -> String {
        self.content.clone()
    }

    #[graphql(name = "done")]
    async fn done_(&self) -> bool {
        self.done
    }

    #[graphql(name = "done")]
    async fn priority_(&self) -> u64 {
        self.priority
    }
}