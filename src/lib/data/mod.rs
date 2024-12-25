pub mod model;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqliteQueryResult, SqliteRow}, Sqlite, SqlitePool};
use thiserror::Error;
use std::str::FromStr;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum DataError{
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error)
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = SqliteRow;
pub type AppQueryResult = SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self{
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(connection_str).await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) =>{ 
                eprintln!("{}\n", e);
                eprintln!("If the database has not yet been created, run: \n $ sqlx database setup\n");
                panic!("Databaze connection error");
        }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}


#[derive(Debug, Clone, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}
