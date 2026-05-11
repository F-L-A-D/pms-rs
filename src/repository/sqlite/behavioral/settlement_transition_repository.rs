use chrono::{
    DateTime,
    Utc,
};

use uuid::Uuid;

use sqlx::{
    Row,
    SqliteConnection,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::domain::settlement_transition::{
    SettlementTransition,
    SettlementTransitionType,
};

impl SettlementTransitionType {

    pub fn as_str(
        &self
    ) -> &'static str {

        match self {

            Self::InvoiceIssued =>
                "InvoiceIssued",

            Self::ReceivableOpened =>
                "ReceivableOpened",
        }
    }

    pub fn from_str(
        value: &str
    ) -> AppResult<Self> {

        match value {

            "InvoiceIssued" =>
                Ok(Self::InvoiceIssued),

            "ReceivableOpened" =>
                Ok(Self::ReceivableOpened),

            other => Err(
                AppError::Infrastructure(
                    format!(
                        "unknown settlement transition type: {}",
                        other
                    )
                )
            )
        }
    }
}

pub struct SqliteSettlementTransitionRepository;

impl SqliteSettlementTransitionRepository {

    pub async fn insert(
        tx: &mut SqliteConnection,
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
            VALUES (?, ?, ?, ?, ?)
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
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        Ok(())
    }

    pub async fn list_by_receivable_id(
        tx: &mut SqliteConnection,
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
                WHERE receivable_id = ?
                ORDER BY occurred_at ASC
                "#
            )
            .bind(
                receivable_id.to_string()
            )
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        let mut transitions =
            Vec::new();

        for row in rows {

            let transition_type =
                SettlementTransitionType::from_str(
                    &row.get::<String, _>(
                        "transition_type"
                    )
                )?;

            transitions.push(
                SettlementTransition {

                    id:
                        Uuid::parse_str(
                            &row.get::<String, _>(
                                "id"
                            )
                        )
                        .map_err(|e| {
                            AppError::Infrastructure(
                                e.to_string()
                            )
                        })?,

                    receivable_id:
                        Uuid::parse_str(
                            &row.get::<String, _>(
                                "receivable_id"
                            )
                        )
                        .map_err(|e| {
                            AppError::Infrastructure(
                                e.to_string()
                            )
                        })?,

                    transition_type,

                    amount:
                        row.get("amount"),

                    occurred_at:
                        DateTime::parse_from_rfc3339(
                            &row.get::<String, _>(
                                "occurred_at"
                            )
                        )
                        .map_err(|e| {
                            AppError::Infrastructure(
                                e.to_string()
                            )
                        })?
                        .with_timezone(&Utc),
                }
            );
        }

        Ok(transitions)
    }
}