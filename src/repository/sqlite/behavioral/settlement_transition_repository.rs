use chrono::{
    DateTime,
    Utc,
};

use uuid::Uuid;

use sqlx::{
    Row,
    Transaction,
    Sqlite,
};

use crate::{
    error::app_error::{
        AppResult,
        infra,
    },

    domain::settlement_transition::{
        SettlementTransition,
        SettlementTransitionType,
    },
};

pub struct SqliteSettlementTransitionRepository;

impl SqliteSettlementTransitionRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        transition: &SettlementTransition,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT INTO settlement_transitions (
                id,
                receivable_id,
                transition_type,
                amount,
                occurred_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#
        )
        .bind(
            transition.id.to_string()
        )
        .bind(
            transition.receivable_id.to_string()
        )
        .bind(
            transition
                .transition_type
                .as_str()
        )
        .bind(
            transition.amount
        )
        .bind(
            transition.occurred_at
                .to_rfc3339()
        )
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<SettlementTransition>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    receivable_id,
                    transition_type,
                    amount,
                    occurred_at
                FROM settlement_transitions
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
                        Self::row_to_transition(
                            &row
                        )?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_by_receivable_id(
        tx: &mut Transaction<'_, Sqlite>,
        receivable_id: Uuid,
    ) -> AppResult<Vec<SettlementTransition>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    receivable_id,
                    transition_type,
                    amount,
                    occurred_at
                FROM settlement_transitions
                WHERE receivable_id = ?1
                ORDER BY occurred_at ASC
                "#
            )
            .bind(
                receivable_id.to_string()
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(
            rows.iter()
                .map(
                    Self::row_to_transition
                )
                .collect::<AppResult<Vec<_>>>()?
        )
    }

    fn row_to_transition(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<SettlementTransition> {

    let transition_type =
        SettlementTransitionType::from_str(
            row.get::<String, _>(
                "transition_type"
            )
            .as_str()
        )
        .map_err(infra)?;

        let occurred_at =
            DateTime::parse_from_rfc3339(
                row.get::<String, _>(
                    "occurred_at"
                )
                .as_str()
            )
            .map_err(infra)?
            .with_timezone(&Utc);

        Ok(
            SettlementTransition {

                id:
                    Uuid::parse_str(
                        row.get::<String, _>(
                            "id"
                        )
                        .as_str()
                    )
                    .map_err(infra)?,

                receivable_id:
                    Uuid::parse_str(
                        row.get::<String, _>(
                            "receivable_id"
                        )
                        .as_str()
                    )
                    .map_err(infra)?,

                transition_type,

                amount:
                    row.get("amount"),

                occurred_at,
            }
        )
    }
}