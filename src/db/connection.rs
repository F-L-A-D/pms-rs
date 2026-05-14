use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Sqlite,
    SqlitePool,
    Transaction,
};

use std::str::FromStr;

use crate::db::bootstrap;

#[derive(Clone)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {

    pub async fn new(
        database_url: &str,
    ) -> Self {

        let options =
            SqliteConnectOptions::from_str(database_url)
                .unwrap()
                .create_if_missing(true);

        let pool =
            SqlitePoolOptions::new()
                .connect_with(options)
                .await
                .unwrap();

        bootstrap::bootstrap(&pool).await;

        Self { pool }
    }

    pub async fn new_test() -> Self {

        let options =
            SqliteConnectOptions::from_str("sqlite::memory:")
                .unwrap()
                .create_if_missing(true);

        let pool =
            SqlitePoolOptions::new()
                .max_connections(1)
                .connect_with(options)
                .await
                .unwrap();

        bootstrap::bootstrap(&pool).await;

        Self { pool }
    }

    pub async fn begin_tx(
        &self,
    ) -> Transaction<'_, Sqlite> {

        self.pool
            .begin()
            .await
            .unwrap()
    }
}