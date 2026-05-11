use chrono::{
    DateTime,
    Utc,
};

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    domain::folio_entry::{
        FolioEntryType,
        FolioEntry,
    },

    error::app_error::{
        AppResult,
        infra,
    }
};

pub struct SqliteFolioEntryRepository;

impl SqliteFolioEntryRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        entry: &FolioEntry,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT INTO folio_entries (
                id,
                folio_id,
                entry_type,
                amount,
                occurred_at,
                description
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#
        )
        .bind(entry.id.to_string())
        .bind(entry.folio_id.to_string())
        .bind(
            match entry.entry_type {

                FolioEntryType::RoomCharge =>
                    "RoomCharge",

                FolioEntryType::Payment =>
                    "Payment",

                FolioEntryType::Adjustment =>
                    "Adjustment",
            }
        )
        .bind(entry.amount)
        .bind(entry.occurred_at.to_rfc3339())
        .bind(&entry.description)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<FolioEntry>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    folio_id,
                    entry_type,
                    amount,
                    occurred_at,
                    description
                FROM folio_entries
                WHERE id = ?1
                "#
            )
            .bind(id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        match row {

            Some(row) => {
                Ok(
                    Some(
                        Self::row_to_entry(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn list_by_folio_id(
        tx: &mut Transaction<'_, Sqlite>,
        folio_id: Uuid,
    ) -> AppResult<Vec<FolioEntry>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    folio_id,
                    entry_type,
                    amount,
                    occurred_at,
                    description
                FROM folio_entries
                WHERE folio_id = ?1
                ORDER BY occurred_at
                "#
            )
            .bind(folio_id.to_string())
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(
            rows.iter()
                .map(Self::row_to_entry)
                .collect::<AppResult<Vec<_>>>()?
        )
    }

    fn row_to_entry(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<FolioEntry> {

        let entry_type =
            match row
                .get::<String, _>("entry_type")
                .as_str()
            {

                "RoomCharge" =>
                    FolioEntryType::RoomCharge,

                "Payment" =>
                    FolioEntryType::Payment,

                "Adjustment" =>
                    FolioEntryType::Adjustment,

                _ => {
                    return Err(
                        infra(
                            "invalid entry type"
                        )
                    )
                }
            };

        let occurred_at =
            DateTime::parse_from_rfc3339(
                row.get::<String, _>("occurred_at")
                    .as_str()
            )
            .map_err(infra)?
            .with_timezone(&Utc);

        Ok(
            FolioEntry {

                id:
                    Uuid::parse_str(
                        row.get::<String, _>("id")
                            .as_str()
                    )
                    .map_err(infra)?,

                folio_id:
                    Uuid::parse_str(
                        row.get::<String, _>("folio_id")
                            .as_str()
                    )
                    .map_err(infra)?,

                entry_type,

                amount:
                    row.get("amount"),

                occurred_at,

                description:
                    row.get("description"),
            }
        )
    }
}