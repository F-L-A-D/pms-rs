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

use crate::domain::reservation::{
    ReservationStatus,
    StayStatus,
};

use crate::error::app_error::AppError;

use crate::projection::operational::
    reservation_search::
    ReservationSearchProjection;

pub struct ReservationSearchProjectionRepository;

impl ReservationSearchProjectionRepository {

    pub async fn upsert(
        tx: &mut Transaction<'_, Sqlite>,
        projection: &ReservationSearchProjection,
    ) -> Result<(), AppError> {

        let participant_names =
            serde_json::to_string(
                &projection.participant_names
            )
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO
            reservation_search_projections (
                reservation_id,
                external_id,
                primary_guest_name,
                participant_names,
                check_in,
                check_out,
                room_class,
                room_id,
                reservation_status,
                stay_status,
                projection_version,
                updated_at
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#
        )
        .bind(
            projection
                .reservation_id
                .to_string()
        )
        .bind(
            &projection.external_id
        )
        .bind(
            &projection.primary_guest_name
        )
        .bind(
            participant_names
        )
        .bind(
            projection.check_in
                .to_string()
        )
        .bind(
            projection.check_out
                .to_string()
        )
        .bind(
            &projection.room_class
        )
        .bind(
            &projection.room_id
        )
        .bind(
            format!(
                "{:?}",
                projection
                    .reservation_status
            )
        )
        .bind(
            projection
                .stay_status
                .as_ref()
                .map(|s| {
                    format!("{:?}", s)
                })
        )
        .bind(
            projection
                .projection_version
        )
        .bind(
            projection
                .updated_at
                .to_rfc3339()
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

    pub async fn find_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> Result<
        Option<ReservationSearchProjection>,
        AppError,
    > {

        let row =
            sqlx::query(
                r#"
                SELECT
                    reservation_id,
                    external_id,
                    primary_guest_name,
                    participant_names,
                    check_in,
                    check_out,
                    room_class,
                    room_id,
                    reservation_status,
                    stay_status,
                    projection_version,
                    updated_at
                FROM reservation_search_projections
                WHERE reservation_id = ?1
                "#
            )
            .bind(
                reservation_id.to_string()
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

        let participant_names =
            serde_json::from_str::<Vec<String>>(
                row.get::<String, _>(
                    "participant_names"
                )
                .as_str()
            )
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        let check_in =
            NaiveDate::parse_from_str(
                row.get::<String, _>(
                    "check_in"
                )
                .as_str(),
                "%Y-%m-%d",
            )
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        let check_out =
            NaiveDate::parse_from_str(
                row.get::<String, _>(
                    "check_out"
                )
                .as_str(),
                "%Y-%m-%d",
            )
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        let reservation_status =
            match row.get::<String, _>(
                "reservation_status"
            )
            .as_str() {

                "Active" => {
                    ReservationStatus::Active
                }

                "Cancelled" => {
                    ReservationStatus::Cancelled
                }

                v => {
                    return Err(
                        AppError::Infrastructure(
                            format!(
                                "invalid reservation_status: {}",
                                v,
                            )
                        )
                    );
                }
            };

        let stay_status =
            match row.get::<Option<String>, _>(
                "stay_status"
            ) {

                Some(v) => {

                    Some(
                        match v.as_str() {

                            "Confirmed" => {
                                StayStatus::Confirmed
                            }

                            "CheckedIn" => {
                                StayStatus::CheckedIn
                            }

                            "CheckedOut" => {
                                StayStatus::CheckedOut
                            }

                            _ => {
                                return Err(
                                    AppError::Infrastructure(
                                        format!(
                                            "invalid stay_status: {}",
                                            v,
                                        )
                                    )
                                );
                            }
                        }
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
                ReservationSearchProjection {

                    reservation_id:
                        Uuid::parse_str(
                            row.get::<String, _>(
                                "reservation_id"
                            )
                            .as_str()
                        )
                        .map_err(|e| {
                            AppError::Infrastructure(
                                e.to_string()
                            )
                        })?,

                    external_id:
                        row.get(
                            "external_id"
                        ),

                    primary_guest_name:
                        row.get(
                            "primary_guest_name"
                        ),

                    participant_names,

                    check_in,

                    check_out,

                    room_class:
                        row.get(
                            "room_class"
                        ),

                    room_id:
                        row.get(
                            "room_id"
                        ),

                    reservation_status,

                    stay_status,

                    projection_version:
                        row.get(
                            "projection_version"
                        ),

                    updated_at,
                }
            )
        )
    }

    pub async fn delete_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> Result<(), AppError> {

        sqlx::query(
            r#"
            DELETE FROM
                reservation_search_projections
            WHERE reservation_id = ?1
            "#
        )
        .bind(
            reservation_id.to_string()
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
            DELETE FROM
                reservation_search_projections
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

    pub async fn find_by_guest_name_partial(
        tx: &mut Transaction<'_, Sqlite>,
        keyword: &str,
    ) -> Result<
        Vec<ReservationSearchProjection>,
        AppError,
    > {

        let pattern =
            format!(
                "%{}%",
                keyword.trim()
            );

        let rows =
            sqlx::query(
                r#"
                SELECT
                    reservation_id
                FROM
                    reservation_search_projections
                WHERE
                    primary_guest_name LIKE ?1
                    OR participant_names LIKE ?1
                ORDER BY
                    check_in ASC
                "#
            )
            .bind(pattern)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        let mut projections =
            vec![];

        for row in rows {

            let reservation_id =
                Uuid::parse_str(
                    row.get::<String, _>(
                        "reservation_id"
                    )
                    .as_str()
                )
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            let projection =
                Self::find_by_reservation_id(
                    tx,
                    reservation_id,
                )
                .await?
                .ok_or(
                    AppError::NotFound(
                        "reservation projection not found"
                            .into()
                    )
                )?;

            projections.push(
                projection
            );
        }

        Ok(
            projections
        )
    }

    pub async fn find_active_by_guest_name_partial(
        tx: &mut Transaction<'_, Sqlite>,
        keyword: &str,
    ) -> Result<
        Vec<ReservationSearchProjection>,
        AppError,
    > {

        let pattern =
            format!(
                "%{}%",
                keyword.trim()
            );

        let rows =
            sqlx::query(
                r#"
                SELECT
                    reservation_id
                FROM
                    reservation_search_projections
                WHERE
                    reservation_status = 'Active'
                    AND (
                        primary_guest_name LIKE ?1
                        OR participant_names LIKE ?1
                    )
                ORDER BY
                    check_in ASC
                "#
            )
            .bind(pattern)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        let mut projections =
            vec![];

        for row in rows {

            let reservation_id =
                Uuid::parse_str(
                    row.get::<String, _>(
                        "reservation_id"
                    )
                    .as_str()
                )
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            let projection =
                Self::find_by_reservation_id(
                    tx,
                    reservation_id,
                )
                .await?
                .ok_or(
                    AppError::NotFound(
                        "reservation projection not found"
                            .into()
                    )
                )?;

            projections.push(
                projection
            );
        }

        Ok(
            projections
        )
    }
}