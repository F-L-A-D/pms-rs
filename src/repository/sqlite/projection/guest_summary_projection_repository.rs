use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::error::app_error::AppError;

use crate::projection::crm::
    guest_summary::GuestSummaryProjection;

pub struct GuestSummaryProjectionRepository;

impl GuestSummaryProjectionRepository {

    pub async fn upsert(
        tx: &mut Transaction<'_, Sqlite>,
        projection: &GuestSummaryProjection,
    ) -> Result<(), AppError> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO guest_summary_projections (
                guest_id,
                total_stays,
                total_nights,
                total_spending,
                last_stay_at,
                projection_version,
                updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(projection.guest_id.to_string())
        .bind(projection.total_stays)
        .bind(projection.total_nights)
        .bind(projection.total_spending)
        .bind(
            projection
                .last_stay_at
                .map(|d| d.to_string())
        )
        .bind(projection.projection_version)
        .bind(projection.updated_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        Ok(())
    }

    pub async fn find_by_guest_id(
        tx: &mut Transaction<'_, Sqlite>,
        guest_id: Uuid,
    ) -> Result<Option<GuestSummaryProjection>, AppError> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    guest_id,
                    total_stays,
                    total_nights,
                    total_spending,
                    last_stay_at,
                    projection_version,
                    updated_at
                FROM guest_summary_projections
                WHERE guest_id = ?1
                "#
            )
            .bind(
                guest_id.to_string()
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        let Some(row) = row else {
            return Ok(None);
        };

        let last_stay_at =
            match row.get::<Option<String>, _>(
                "last_stay_at"
            ) {

                Some(v) => {
                    Some(
                        NaiveDate::parse_from_str(
                            &v,
                            "%Y-%m-%d",
                        )
                        .map_err(|e| {
                            AppError::Infrastructure(
                                e.to_string()
                            )
                        })?
                    )
                }

                None => None,
            };

        let updated_at =
            DateTime::parse_from_rfc3339(
                row.get::<String, _>(
                    "updated_at"
                )
                .as_str()
            )
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?
            .with_timezone(&Utc);

        Ok(
            Some(
                GuestSummaryProjection {

                    guest_id:
                        Uuid::parse_str(
                            row.get::<String, _>(
                                "guest_id"
                            )
                            .as_str()
                        )
                        .map_err(|e| {
                            AppError::Infrastructure(
                                e.to_string()
                            )
                        })?,

                    total_stays:
                        row.get(
                            "total_stays"
                        ),

                    total_nights:
                        row.get(
                            "total_nights"
                        ),

                    total_spending:
                        row.get(
                            "total_spending"
                        ),

                    last_stay_at,

                    projection_version:
                        row.get(
                            "projection_version"
                        ),

                    updated_at,
                }
            )
        )
    }

    pub async fn delete_by_guest_id(
        tx: &mut Transaction<'_, Sqlite>,
        guest_id: Uuid,
    ) -> Result<(), AppError> {

        sqlx::query(
            r#"
            DELETE FROM guest_summary_projections
            WHERE guest_id = ?1
            "#
        )
        .bind(
            guest_id.to_string()
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        Ok(())
    }

    pub async fn delete_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), AppError> {

        sqlx::query(
            r#"
            DELETE FROM guest_summary_projections
            "#
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        Ok(())
    }
}