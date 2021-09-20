use anyhow::Result;
use std::env;

pub(super) struct Credentials {
    pub user: String,
    pub password: String,
    pub host: String,
    pub db_name: String,
}

impl Credentials {
    pub(super) fn from_environment() -> Result<Self> {
        Ok(Self {
            user: env::var("DB_USER")?,
            password: env::var("DB_PASSWORD")?,
            host: env::var("DB_HOST")?,
            db_name: env::var("DB_NAME")?,
        })
    }
}
