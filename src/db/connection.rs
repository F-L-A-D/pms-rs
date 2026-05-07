use sqlx::{
    Sqlite, 
    SqlitePool, 
    Transaction, 
    sqlite::{
        SqliteConnectOptions, 
        SqlitePoolOptions,
    },
};

use std::str::FromStr;

#[derive(Clone)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn new(database_url: &str) -> Self {
        let options = 
            SqliteConnectOptions::from_str(database_url)
                .unwrap()
                .create_if_missing(true);

        let pool = 
            SqlitePoolOptions::new()
                .connect_with(options)
                .await
                .unwrap();

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

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS folio_entries (
                id TEXT PRIMARY KEY,
                folio_id TEXT NOT NULL,
                entry_type TEXT NOT NULL,
                amount INTEGER NOT NULL,
                occurred_at TEXT NOT NULL,
                description TEXT
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS guests (
                id TEXT PRIMARY KEY,
                last_name TEXT NOT NULL,
                first_name TEXT NOT NULL,
                phone TEXT,
                email TEXT,
                nationality TEXT,
                birth_date TEXT,
                gender TEXT,
                membership_code TEXT,
                marketing_opt_in INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS reservation_guest_relations (
                reservation_id TEXT NOT NULL,
                guest_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                UNIQUE(reservation_id, guest_id)
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS guest_timeline_events (
                id TEXT PRIMARY KEY,
                guest_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                reference_id TEXT NOT NULL,
                occurred_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_guest_timeline_guest
            ON guest_timeline_events (
                guest_id,
                occurred_at DESC
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