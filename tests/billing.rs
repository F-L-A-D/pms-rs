use chrono::Utc;

use rust_decimal::Decimal;

use uuid::Uuid;

use pms_rs::{
    api::dto::billing::input::{
        assign_billing_account_input::AssignBillingAccountInput,
        close_folio_input::CloseFolioInput, create_folio_entry_input::CreateFolioEntryInput,
        create_invoice_input::CreateInvoiceInput, create_payment_input::CreatePaymentInput,
    },
    db::connection::Db,
    domain::entity::{
        billing_account::{BillingAccount, BillingAccountStatus},
        folio::{Folio, FolioStatus},
        folio_entry::FolioEntryType,
        payment::PaymentMethod,
    },
    domain::semantic::settlement_transition::SettlementTransitionType,
    error::app_error::AppError,
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::{
            billing_account_repository::SqliteBillingAccountRepository,
            folio_repository::SqliteFolioRepository, invoice_repository::SqliteInvoiceRepository,
            receivable_repository::SqliteReceivableRepository,
        },
    },
    usecase::billing::command::{
        assign_billing_account, close_folio, create_folio_entry, create_invoice, create_payment,
    },
};

#[tokio::test]
async fn should_issue_invoice_and_create_receivable_and_settlement_history() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Open, Some(billing_account_id)).await;

    let closed_folio = close_folio::execute(&db, CloseFolioInput { folio_id })
        .await
        .unwrap();

    assert_eq!(closed_folio.status, FolioStatus::Closed);

    let invoice = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-001".to_string(),
            issued_amount: Decimal::new(12500, 2),
        },
    )
    .await
    .unwrap();

    assert_eq!(invoice.folio_id, folio_id);
    assert_eq!(invoice.billing_account_id, billing_account_id);
    assert_eq!(invoice.invoice_number, "INV-001");

    let mut tx = db.begin_tx().await;

    let persisted_invoice = SqliteInvoiceRepository::find_by_folio_id(&mut tx, folio_id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(persisted_invoice.id, invoice.id);

    let receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(receivable.invoice_id, invoice.id);
    assert_eq!(receivable.outstanding_amount, invoice.issued_amount);

    let transitions =
        SqliteSettlementTransitionRepository::find_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();

    assert_eq!(transitions.len(), 2);
    assert_eq!(
        transitions[0].transition_type,
        SettlementTransitionType::InvoiceIssued,
    );
    assert_eq!(
        transitions[1].transition_type,
        SettlementTransitionType::ReceivableOpened,
    );

    let _ = tx.rollback().await;
}

#[tokio::test]
async fn should_reject_invoice_for_open_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Open, Some(billing_account_id)).await;

    let result = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-OPEN".to_string(),
            issued_amount: Decimal::new(10000, 2),
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
    assert_no_invoice_for_folio(&db, folio_id).await;
}

#[tokio::test]
async fn should_reject_invoice_for_locked_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Locked, Some(billing_account_id)).await;

    let result = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-LOCKED".to_string(),
            issued_amount: Decimal::new(10000, 2),
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
    assert_no_invoice_for_folio(&db, folio_id).await;
}

#[tokio::test]
async fn should_reject_invoice_when_billing_account_is_not_assigned() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Closed, None).await;

    let result = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-NO-ACCOUNT".to_string(),
            issued_amount: Decimal::new(10000, 2),
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
    assert_no_invoice_for_folio(&db, folio_id).await;
}

#[tokio::test]
async fn should_reject_duplicate_invoice_for_same_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Closed, Some(billing_account_id)).await;

    create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-FIRST".to_string(),
            issued_amount: Decimal::new(10000, 2),
        },
    )
    .await
    .unwrap();

    let result = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-SECOND".to_string(),
            issued_amount: Decimal::new(10000, 2),
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_assign_billing_account_to_open_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Open, None).await;

    let folio = assign_billing_account::execute(
        &db,
        AssignBillingAccountInput {
            folio_id,
            billing_account_id,
        },
    )
    .await
    .unwrap();

    assert_eq!(folio.billing_account_id, Some(billing_account_id));
}

#[tokio::test]
async fn should_reject_billing_account_assignment_for_locked_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Locked, None).await;

    let result = assign_billing_account::execute(
        &db,
        AssignBillingAccountInput {
            folio_id,
            billing_account_id,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_reject_billing_account_assignment_for_closed_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Closed, None).await;

    let result = assign_billing_account::execute(
        &db,
        AssignBillingAccountInput {
            folio_id,
            billing_account_id,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_close_open_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Open, None).await;

    let folio = close_folio::execute(&db, CloseFolioInput { folio_id })
        .await
        .unwrap();

    assert_eq!(folio.status, FolioStatus::Closed);
}

#[tokio::test]
async fn should_close_locked_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Locked, None).await;

    let folio = close_folio::execute(&db, CloseFolioInput { folio_id })
        .await
        .unwrap();

    assert_eq!(folio.status, FolioStatus::Closed);
}

#[tokio::test]
async fn should_reject_closing_already_closed_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Closed, None).await;

    let result = close_folio::execute(&db, CloseFolioInput { folio_id }).await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_reject_folio_entry_for_locked_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Locked, None).await;

    let result = create_folio_entry::execute(
        &db,
        CreateFolioEntryInput {
            folio_id,
            entry_type: FolioEntryType::RoomCharge,
            amount: Decimal::new(10000, 2),
            memo: None,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Domain(_))));
}

#[tokio::test]
async fn should_reject_folio_entry_for_closed_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Closed, None).await;

    let result = create_folio_entry::execute(
        &db,
        CreateFolioEntryInput {
            folio_id,
            entry_type: FolioEntryType::RoomCharge,
            amount: Decimal::new(10000, 2),
            memo: None,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Domain(_))));
}

#[tokio::test]
async fn should_reject_payment_for_locked_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Locked, None).await;

    let result = create_payment::execute(
        &db,
        CreatePaymentInput {
            folio_id,
            amount: Decimal::new(10000, 2),
            method: PaymentMethod::Cash,
            external_reference: None,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Domain(_))));
}

#[tokio::test]
async fn should_reject_payment_for_closed_folio() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();

    seed_folio(&db, folio_id, FolioStatus::Closed, None).await;

    let result = create_payment::execute(
        &db,
        CreatePaymentInput {
            folio_id,
            amount: Decimal::new(10000, 2),
            method: PaymentMethod::Cash,
            external_reference: None,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Domain(_))));
}

async fn seed_billing_account(db: &Db, billing_account_id: Uuid) {
    let mut tx = db.begin_tx().await;

    let account = BillingAccount {
        id: billing_account_id,
        company_id: None,
        name: format!("billing-account-{billing_account_id}"),
        status: BillingAccountStatus::Active,
        created_at: Utc::now(),
    };

    SqliteBillingAccountRepository::save(&mut tx, &account)
        .await
        .unwrap();

    tx.commit().await.unwrap();
}

async fn seed_folio(
    db: &Db,
    folio_id: Uuid,
    status: FolioStatus,
    billing_account_id: Option<Uuid>,
) {
    let mut tx = db.begin_tx().await;

    let folio = Folio {
        id: folio_id,
        reservation_id: Uuid::new_v4(),
        billing_account_id,
        status,
        created_at: Utc::now(),
    };

    SqliteFolioRepository::save(&mut tx, &folio).await.unwrap();

    tx.commit().await.unwrap();
}

async fn assert_no_invoice_for_folio(db: &Db, folio_id: Uuid) {
    let mut tx = db.begin_tx().await;

    let invoice = SqliteInvoiceRepository::find_by_folio_id(&mut tx, folio_id)
        .await
        .unwrap();

    assert!(invoice.is_none());

    let _ = tx.rollback().await;
}
