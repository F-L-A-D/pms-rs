use sqlx::{SqlitePool, Sqlite, Transaction};

pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn new(database_url: &str) -> Self {
        let pool = SqlitePool::connect(database_url).await.unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS reservations (
                id TEXT PRIMARY KEY,
                check_in TEXT NOT NULL,
                check_out TEXT NOT NULL,
                reservation_status TEXT NOT NULL,
                stay_status TEXT,
                room_class TEXT NOT NULL,
                room_id TEXT,
                created_at TEXT NOT NULL,
                channel TEXT
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS inventory (
                date TEXT PRIMARY KEY,
                total_rooms INTEGER NOT NULL,
                reserved_rooms INTEGER NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS rooms (
                id TEXT PRIMARY KEY,
                room_class TEXT NOT NULL,
                occupancy_status TEXT NOT NULL,
                housekeeping_status TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS folios (
                id TEXT PRIMARY KEY,
                reservation_id TEXT NOT NULL,
                status TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        Self { pool }
    }

    pub async fn begin_tx(&self) -> Transaction<'_, Sqlite> {
        self.pool.begin().await.unwrap()
    }
}