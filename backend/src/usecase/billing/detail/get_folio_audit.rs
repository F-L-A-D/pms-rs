use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    api::dto::billing::response::billing_audit_response::BillingAuditResponse,
    db::connection::Db,
    domain::{
        entity::payment::PaymentMethod, semantic::operation_change_event::OperationChangeEvent,
    },
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::{
        billing::{
            billing_account_repository::SqliteBillingAccountRepository,
            deposit_application_repository::SqliteDepositApplicationRepository,
            deposit_repository::SqliteDepositRepository,
            invoice_repository::SqliteInvoiceRepository,
            payment_allocation_repository::SqlitePaymentAllocationRepository,
            payment_repository::SqlitePaymentRepository,
            receivable_repository::SqliteReceivableRepository,
        },
        operation::{
            operation_change_event_repository::SqliteOperationChangeEventRepository,
            operational_audit_log_repository::SqliteOperationalAuditLogRepository,
        },
    },
};

pub async fn execute(db: &Db, folio_id: Uuid) -> AppResult<Vec<BillingAuditResponse>> {
    let mut tx = db.begin_tx().await;

    let events = SqliteOperationChangeEventRepository::list_all(&mut tx).await?;

    let mut responses = Vec::new();

    for event in events {
        let resolved_folio_id = resolve_event_folio_id(&mut tx, &event).await?;

        if resolved_folio_id != Some(folio_id) {
            continue;
        }

        let audit_extract = extract_billing_audit(&event.after_json)?;

        let audit_logs =
            SqliteOperationalAuditLogRepository::list_by_operation_id(&mut tx, event.operation_id)
                .await?;

        let audit = audit_logs.first();

        let billing_account_name = match audit_extract.billing_account_id {
            Some(billing_account_id) => {
                SqliteBillingAccountRepository::find_by_id(&mut tx, billing_account_id)
                    .await?
                    .map(|account| account.name)
            }
            None => None,
        };

        responses.push(BillingAuditResponse {
            operation_id: event.operation_id,

            folio_id: resolved_folio_id,
            folio_entry_id: audit_extract.folio_entry_id,
            payment_id: audit_extract.payment_id,
            invoice_id: audit_extract.invoice_id,

            aggregate_type: event.aggregate_type,
            aggregate_id: event.aggregate_id,

            operation_type: event.operation_type,
            actor: event.actor,
            actor_id: event.actor_id,
            source: event.source,

            action: audit.map(|log| log.action.clone()),
            reason: audit.and_then(|log| log.reason.clone()),

            amount: audit_extract.amount,
            payment_method: audit_extract.payment_method,
            payment_reference: audit_extract.payment_reference,

            invoice_number: audit_extract.invoice_number,
            issued_amount: audit_extract.issued_amount,

            billing_account_id: audit_extract.billing_account_id,
            billing_account_name,

            before_json: event.before_json,
            after_json: event.after_json,
            changed_fields_json: event.changed_fields_json,

            occurred_at: event.occurred_at,
        });
    }

    responses.sort_by_key(|item| item.occurred_at);

    tx.commit().await.map_err(infra)?;

    Ok(responses)
}

async fn resolve_event_folio_id(
    tx: &mut Transaction<'_, Sqlite>,
    event: &OperationChangeEvent,
) -> AppResult<Option<Uuid>> {
    match event.aggregate_type.as_str() {
        "folio" => Ok(Some(event.aggregate_id)),

        "invoice" => {
            let invoice = SqliteInvoiceRepository::find_by_id(tx, event.aggregate_id).await?;

            Ok(invoice.map(|invoice| invoice.folio_id))
        }

        "receivable" => {
            let receivable = SqliteReceivableRepository::find_by_id(tx, event.aggregate_id).await?;

            let Some(receivable) = receivable else {
                return Ok(None);
            };

            let invoice = SqliteInvoiceRepository::find_by_id(tx, receivable.invoice_id).await?;

            Ok(invoice.map(|invoice| invoice.folio_id))
        }

        "payment_allocation" => {
            let allocation =
                SqlitePaymentAllocationRepository::find_by_id(tx, event.aggregate_id).await?;

            let Some(allocation) = allocation else {
                return Ok(None);
            };

            let receivable =
                SqliteReceivableRepository::find_by_id(tx, allocation.receivable_id).await?;

            let Some(receivable) = receivable else {
                return Ok(None);
            };

            let invoice = SqliteInvoiceRepository::find_by_id(tx, receivable.invoice_id).await?;

            Ok(invoice.map(|invoice| invoice.folio_id))
        }

        "deposit_application" => {
            let application =
                SqliteDepositApplicationRepository::find_by_id(tx, event.aggregate_id).await?;

            let Some(application) = application else {
                return Ok(None);
            };

            let deposit = SqliteDepositRepository::find_by_id(tx, application.deposit_id).await?;

            Ok(deposit.map(|deposit| deposit.folio_id))
        }

        "payment" => {
            let payment = SqlitePaymentRepository::find_by_id(tx, event.aggregate_id).await?;

            Ok(payment.map(|payment| payment.folio_id))
        }

        "deposit" => {
            let deposit = SqliteDepositRepository::find_by_id(tx, event.aggregate_id).await?;

            Ok(deposit.map(|deposit| deposit.folio_id))
        }

        _ => Ok(None),
    }
}

#[derive(Debug, Default)]
struct BillingAuditExtract {
    folio_id: Option<Uuid>,
    folio_entry_id: Option<Uuid>,
    payment_id: Option<Uuid>,
    invoice_id: Option<Uuid>,

    amount: Option<Decimal>,
    payment_method: Option<PaymentMethod>,
    payment_reference: Option<String>,

    invoice_number: Option<String>,
    issued_amount: Option<Decimal>,

    billing_account_id: Option<Uuid>,
}

fn extract_billing_audit(json: &str) -> AppResult<BillingAuditExtract> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(infra)?;

    let amount = read_decimal(&value, "amount")?.or(read_decimal(&value, "payment_amount")?);

    let payment_method =
        read_payment_method(&value, "method")?.or(read_payment_method(&value, "payment_method")?);

    let payment_reference = read_string(&value, "external_reference")
        .or_else(|| read_string(&value, "payment_reference"));

    Ok(BillingAuditExtract {
        folio_id: read_uuid(&value, "folio_id")?,
        folio_entry_id: read_uuid(&value, "folio_entry_id")?,
        payment_id: read_uuid(&value, "payment_id")?,
        invoice_id: read_uuid(&value, "invoice_id")?,

        amount,
        payment_method,
        payment_reference,

        invoice_number: read_string(&value, "invoice_number"),
        issued_amount: read_decimal(&value, "issued_amount")?,

        billing_account_id: read_uuid(&value, "billing_account_id")?,
    })
}

fn read_uuid(value: &serde_json::Value, key: &str) -> AppResult<Option<Uuid>> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(Uuid::parse_str)
        .transpose()
        .map_err(infra)
}

fn read_string(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
}

fn read_decimal(value: &serde_json::Value, key: &str) -> AppResult<Option<Decimal>> {
    value
        .get(key)
        .and_then(|v| {
            if let Some(s) = v.as_str() {
                Some(s.to_string())
            } else if v.is_number() {
                Some(v.to_string())
            } else {
                None
            }
        })
        .map(|raw| Decimal::from_str_exact(&raw))
        .transpose()
        .map_err(infra)
}

fn read_payment_method(value: &serde_json::Value, key: &str) -> AppResult<Option<PaymentMethod>> {
    let Some(raw) = value.get(key).and_then(|v| v.as_str()) else {
        return Ok(None);
    };

    PaymentMethod::from_snake(raw)
        .ok_or_else(|| infra("invalid payment method"))
        .map(Some)
}
