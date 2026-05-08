use chrono::{
    DateTime,
    Utc,
};

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use crate::domain::folio_entry::{
    EntryType,
    FolioEntry,
};

pub struct SqliteFolioEntryRepository;

impl SqliteFolioEntryRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        entry: &FolioEntry,
    ) -> Result<(), String> {

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
        .bind(&entry.id)
        .bind(&entry.folio_id)
        .bind(format!("{:?}", entry.entry_type))
        .bind(entry.amount)
        .bind(entry.occurred_at.to_rfc3339())
        .bind(&entry.description)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_folio_id(
        tx: &mut Transaction<'_, Sqlite>,
        folio_id: &str,
    ) -> Result<Vec<FolioEntry>, String> {

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
            .bind(folio_id)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        let mut entries = vec![];

        for r in rows {

            let entry_type =
                match r.get::<String, _>("entry_type").as_str() {

                    "RoomCharge" =>
                        EntryType::RoomCharge,

                    "Payment" =>
                        EntryType::Payment,

                    "Adjustment" =>
                        EntryType::Adjustment,

                    _ => {
                        return Err(
                            "invalid entry type".into()
                        )
                    }
                };

            let occurred_at_str =
                r.get::<String, _>("occurred_at");

            let occurred_at =
                DateTime::parse_from_rfc3339(
                    &occurred_at_str,
                )
                .map_err(|e| e.to_string())?
                .with_timezone(&Utc);

            entries.push(
                FolioEntry {
                    id: r.get("id"),

                    folio_id:
                        r.get("folio_id"),

                    entry_type,

                    amount:
                        r.get("amount"),

                    occurred_at,

                    description:
                        r.get("description"),
                }
            );
        }

        Ok(entries)
    }
}