use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    domain::folio::{
        Folio,
        FolioStatus,
    },

    error::app_error::{
        AppResult,
        infra,
    },
};

pub struct SqliteFolioRepository;

impl SqliteFolioRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        folio: &Folio,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO folios (
                id,
                reservation_id,
                billing_account_id,
                status
            )
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(folio.id.to_string())
        .bind(folio.reservation_id.to_string())
        .bind(
            folio.billing_account_id
                .map(|id| id.to_string())
        )
        .bind(
            match folio.status {

                FolioStatus::Open =>
                    "Open",

                FolioStatus::Closed =>
                    "Closed",
            }
        )
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Folio>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    reservation_id,
                    billing_account_id,
                    status
                FROM folios
                WHERE id = ?1
                "#,
            )
            .bind(id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        match row {

            Some(row) => {
                Ok(
                    Some(
                        Self::row_to_folio(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<Folio>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    reservation_id,
                    billing_account_id,
                    status
                FROM folios
                WHERE reservation_id = ?1
                "#,
            )
            .bind(reservation_id.to_string())
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(
            rows.iter()
                .map(Self::row_to_folio)
                .collect::<AppResult<Vec<_>>>()?
        )
    }

    fn row_to_folio(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<Folio> {

        let status =
            match row
                .get::<String, _>("status")
                .as_str()
            {
                "Open" =>
                    FolioStatus::Open,

                "Closed" =>
                    FolioStatus::Closed,

                _ =>
                    return Err(
                        infra(
                            "invalid folio status"
                        )
                    ),
            };

        Ok(
            Folio {

                id:
                    Uuid::parse_str(
                        row.get::<String, _>("id")
                            .as_str()
                    )
                    .map_err(infra)?,

                reservation_id:
                    Uuid::parse_str(
                        row.get::<String, _>(
                            "reservation_id"
                        )
                        .as_str()
                    )
                    .map_err(infra)?,

                billing_account_id:
                    row.get::<Option<String>, _>(
                        "billing_account_id"
                    )
                    .map(|s| {
                        Uuid::parse_str(&s)
                    })
                    .transpose()
                    .map_err(infra)?,

                status,
            }
        )
    }
}