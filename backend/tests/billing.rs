use chrono::Utc;

use rust_decimal::Decimal;

use uuid::Uuid;

use pms_rs::{
    api::dto::billing::input::{
        allocate_receivable_payment_input::AllocateReceivablePaymentInput,
        assign_billing_account_input::AssignBillingAccountInput,
        close_folio_input::CloseFolioInput, create_folio_entry_input::CreateFolioEntryInput,
        create_invoice_input::CreateInvoiceInput, create_payment_input::CreatePaymentInput,
    },
    db::connection::Db,
    domain::entity::{
        billing_account::{BillingAccount, BillingAccountStatus},
        folio::{Folio, FolioStatus},
        folio_entry::FolioEntryType,
        invoice::InvoiceStatus,
        payment::PaymentMethod,
        receivable::ReceivableStatus,
    },
    domain::semantic::settlement_transition::SettlementTransitionType,
    error::app_error::AppError,
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::billing::{
            billing_account_repository::SqliteBillingAccountRepository,
            folio_repository::SqliteFolioRepository, invoice_repository::SqliteInvoiceRepository,
            payment_allocation_repository::SqlitePaymentAllocationRepository,
            receivable_repository::SqliteReceivableRepository,
        },
    },
    usecase::billing::command::{
        account::assign_billing_account,
        folio::{close_folio, create_folio_entry},
        invoice::{create_invoice, void_invoice},
        payment::{allocate_receivable_payment, create_payment, reverse_payment_allocation},
        receivable::{dispute_receivable, resolve_receivable_dispute, write_off_receivable},
    },
    usecase::billing::search::list_receivable_aging,
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
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
        },
    )
    .await
    .unwrap();

    assert_eq!(invoice.folio_id, folio_id);
    assert_eq!(invoice.billing_account_id, billing_account_id);
    assert_eq!(invoice.invoice_number, "INV-001");
    assert_eq!(
        invoice.due_date,
        Utc::now().date_naive() + chrono::Duration::days(30)
    );

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
    assert_eq!(receivable.due_date, invoice.due_date);

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
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
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
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
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
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
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
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
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
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_reject_duplicate_invoice_number_across_folios() {
    let db = Db::new_test().await;
    let folio_id_1 = Uuid::new_v4();
    let folio_id_2 = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(
        &db,
        folio_id_1,
        FolioStatus::Closed,
        Some(billing_account_id),
    )
    .await;
    seed_folio(
        &db,
        folio_id_2,
        FolioStatus::Closed,
        Some(billing_account_id),
    )
    .await;

    create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id: folio_id_1,
            invoice_number: "INV-DUPLICATE-NUMBER".to_string(),
            issued_amount: Decimal::new(10000, 2),
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
        },
    )
    .await
    .unwrap();

    let result = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id: folio_id_2,
            invoice_number: "INV-DUPLICATE-NUMBER".to_string(),
            issued_amount: Decimal::new(10000, 2),
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_allocate_partial_payment_to_receivable() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Closed, Some(billing_account_id)).await;

    let invoice = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-PARTIAL".to_string(),
            issued_amount: Decimal::new(10000, 2),
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
        },
    )
    .await
    .unwrap();

    let mut tx = db.begin_tx().await;
    let receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
        .await
        .unwrap()
        .unwrap();
    let _ = tx.rollback().await;

    let result = allocate_receivable_payment::execute(
        &db,
        AllocateReceivablePaymentInput {
            receivable_id: receivable.id,
            amount: Decimal::new(4000, 2),
            method: PaymentMethod::BankTransfer,
            external_reference: Some("BANK-001".to_string()),
            reason: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(result.remaining_outstanding_amount, Decimal::new(6000, 2));

    let mut tx = db.begin_tx().await;
    let updated = SqliteReceivableRepository::find_by_id(&mut tx, receivable.id)
        .await
        .unwrap()
        .unwrap();
    let allocations =
        SqlitePaymentAllocationRepository::list_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let transitions =
        SqliteSettlementTransitionRepository::find_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let _ = tx.rollback().await;

    assert_eq!(updated.outstanding_amount, Decimal::new(6000, 2));
    assert_eq!(updated.status, ReceivableStatus::Open);
    assert_eq!(allocations.len(), 1);
    assert_eq!(allocations[0].amount, Decimal::new(4000, 2));
    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::PaymentAllocated
            && transition.amount == Decimal::new(4000, 2)
    }));
}

#[tokio::test]
async fn should_settle_receivable_when_payment_covers_outstanding_amount() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Closed, Some(billing_account_id)).await;

    let invoice = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-SETTLED".to_string(),
            issued_amount: Decimal::new(7500, 2),
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
        },
    )
    .await
    .unwrap();

    let mut tx = db.begin_tx().await;
    let receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
        .await
        .unwrap()
        .unwrap();
    let _ = tx.rollback().await;

    allocate_receivable_payment::execute(
        &db,
        AllocateReceivablePaymentInput {
            receivable_id: receivable.id,
            amount: Decimal::new(7500, 2),
            method: PaymentMethod::CreditCard,
            external_reference: None,
            reason: None,
        },
    )
    .await
    .unwrap();

    let mut tx = db.begin_tx().await;
    let updated = SqliteReceivableRepository::find_by_id(&mut tx, receivable.id)
        .await
        .unwrap()
        .unwrap();
    let transitions =
        SqliteSettlementTransitionRepository::find_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let _ = tx.rollback().await;

    assert_eq!(updated.outstanding_amount, Decimal::ZERO);
    assert_eq!(updated.status, ReceivableStatus::Settled);
    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::ReceivableSettled
    }));
}

#[tokio::test]
async fn should_reverse_payment_allocation_and_reopen_receivable() {
    let db = Db::new_test().await;
    let receivable = seed_invoiced_receivable(&db, "INV-REVERSE", Decimal::new(10000, 2)).await;

    let result = allocate_receivable_payment::execute(
        &db,
        AllocateReceivablePaymentInput {
            receivable_id: receivable.id,
            amount: Decimal::new(10000, 2),
            method: PaymentMethod::BankTransfer,
            external_reference: Some("BANK-REV".to_string()),
            reason: None,
        },
    )
    .await
    .unwrap();

    reverse_payment_allocation::execute(&db, result.allocation.id, None)
        .await
        .unwrap();

    let mut tx = db.begin_tx().await;
    let updated = SqliteReceivableRepository::find_by_id(&mut tx, receivable.id)
        .await
        .unwrap()
        .unwrap();
    let allocations =
        SqlitePaymentAllocationRepository::list_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let transitions =
        SqliteSettlementTransitionRepository::find_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let _ = tx.rollback().await;

    assert_eq!(updated.status, ReceivableStatus::Open);
    assert_eq!(updated.outstanding_amount, Decimal::new(10000, 2));
    assert!(allocations[0].reversed_at.is_some());
    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::PaymentAllocationReversed
            && transition.amount == Decimal::new(10000, 2)
    }));
}

#[tokio::test]
async fn should_void_invoice_without_active_payment_allocations() {
    let db = Db::new_test().await;
    let invoice = seed_invoice(&db, "INV-VOID", Decimal::new(6000, 2), 30).await;

    let voided = void_invoice::execute(&db, invoice.id, None).await.unwrap();

    assert_eq!(voided.status, InvoiceStatus::Voided);

    let mut tx = db.begin_tx().await;
    let receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
        .await
        .unwrap()
        .unwrap();
    let transitions =
        SqliteSettlementTransitionRepository::find_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let _ = tx.rollback().await;

    assert_eq!(receivable.status, ReceivableStatus::Voided);
    assert_eq!(receivable.outstanding_amount, Decimal::ZERO);
    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::InvoiceVoided
    }));
    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::ReceivableVoided
    }));
}

#[tokio::test]
async fn should_reject_invoice_void_with_active_payment_allocation() {
    let db = Db::new_test().await;
    let invoice = seed_invoice(&db, "INV-VOID-PAID", Decimal::new(6000, 2), 30).await;

    let mut tx = db.begin_tx().await;
    let receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
        .await
        .unwrap()
        .unwrap();
    let _ = tx.rollback().await;

    allocate_receivable_payment::execute(
        &db,
        AllocateReceivablePaymentInput {
            receivable_id: receivable.id,
            amount: Decimal::new(1000, 2),
            method: PaymentMethod::Cash,
            external_reference: None,
            reason: None,
        },
    )
    .await
    .unwrap();

    let result = void_invoice::execute(&db, invoice.id, None).await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_report_receivable_aging_buckets() {
    let db = Db::new_test().await;
    let as_of_date = Utc::now().date_naive();

    seed_invoice(&db, "INV-AGING-CURRENT", Decimal::new(1000, 2), 10).await;
    seed_invoice(&db, "INV-AGING-15", Decimal::new(2000, 2), -15).await;
    seed_invoice(&db, "INV-AGING-45", Decimal::new(3000, 2), -45).await;
    seed_invoice(&db, "INV-AGING-75", Decimal::new(4000, 2), -75).await;
    seed_invoice(&db, "INV-AGING-120", Decimal::new(5000, 2), -120).await;

    let aging = list_receivable_aging::execute(&db, as_of_date)
        .await
        .unwrap();

    assert_eq!(aging.current_amount, Decimal::new(1000, 2));
    assert_eq!(aging.overdue_1_30_amount, Decimal::new(2000, 2));
    assert_eq!(aging.overdue_31_60_amount, Decimal::new(3000, 2));
    assert_eq!(aging.overdue_61_90_amount, Decimal::new(4000, 2));
    assert_eq!(aging.overdue_90_plus_amount, Decimal::new(5000, 2));
    assert_eq!(aging.total_open_amount, Decimal::new(15000, 2));
}

#[tokio::test]
async fn should_reject_receivable_overpayment() {
    let db = Db::new_test().await;
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(&db, billing_account_id).await;
    seed_folio(&db, folio_id, FolioStatus::Closed, Some(billing_account_id)).await;

    let invoice = create_invoice::execute(
        &db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: "INV-OVERPAY".to_string(),
            issued_amount: Decimal::new(5000, 2),
            due_date: Utc::now().date_naive() + chrono::Duration::days(30),
        },
    )
    .await
    .unwrap();

    let mut tx = db.begin_tx().await;
    let receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
        .await
        .unwrap()
        .unwrap();
    let _ = tx.rollback().await;

    let result = allocate_receivable_payment::execute(
        &db,
        AllocateReceivablePaymentInput {
            receivable_id: receivable.id,
            amount: Decimal::new(5001, 2),
            method: PaymentMethod::Cash,
            external_reference: None,
            reason: None,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn should_dispute_and_resolve_receivable() {
    let db = Db::new_test().await;
    let receivable = seed_invoiced_receivable(&db, "INV-DISPUTE", Decimal::new(10000, 2)).await;

    let disputed = dispute_receivable::execute(&db, receivable.id, None)
        .await
        .unwrap();

    assert_eq!(disputed.status, ReceivableStatus::Disputed);

    let resolved = resolve_receivable_dispute::execute(&db, receivable.id, None)
        .await
        .unwrap();

    assert_eq!(resolved.status, ReceivableStatus::Open);

    let mut tx = db.begin_tx().await;
    let transitions =
        SqliteSettlementTransitionRepository::find_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let _ = tx.rollback().await;

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::ReceivableDisputed
    }));
    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::ReceivableDisputeResolved
    }));
}

#[tokio::test]
async fn should_write_off_open_receivable() {
    let db = Db::new_test().await;
    let receivable = seed_invoiced_receivable(&db, "INV-WRITE-OFF", Decimal::new(4300, 2)).await;

    let written_off = write_off_receivable::execute(&db, receivable.id, None)
        .await
        .unwrap();

    assert_eq!(written_off.status, ReceivableStatus::WrittenOff);
    assert_eq!(written_off.outstanding_amount, Decimal::ZERO);

    let mut tx = db.begin_tx().await;
    let transitions =
        SqliteSettlementTransitionRepository::find_by_receivable_id(&mut tx, receivable.id)
            .await
            .unwrap();
    let _ = tx.rollback().await;

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == SettlementTransitionType::ReceivableWrittenOff
            && transition.amount == Decimal::new(4300, 2)
    }));
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
            reason: None,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
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
            reason: None,
        },
    )
    .await;

    assert!(matches!(result, Err(AppError::Conflict(_))));
}

async fn seed_invoiced_receivable(
    db: &Db,
    invoice_number: &str,
    issued_amount: Decimal,
) -> pms_rs::domain::entity::receivable::Receivable {
    let invoice = seed_invoice(db, invoice_number, issued_amount, 30).await;

    let mut tx = db.begin_tx().await;
    let receivable = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
        .await
        .unwrap()
        .unwrap();
    let _ = tx.rollback().await;

    receivable
}

async fn seed_invoice(
    db: &Db,
    invoice_number: &str,
    issued_amount: Decimal,
    due_in_days: i64,
) -> pms_rs::domain::entity::invoice::Invoice {
    let folio_id = Uuid::new_v4();
    let billing_account_id = Uuid::new_v4();

    seed_billing_account(db, billing_account_id).await;
    seed_folio(db, folio_id, FolioStatus::Closed, Some(billing_account_id)).await;

    create_invoice::execute(
        db,
        CreateInvoiceInput {
            folio_id,
            invoice_number: invoice_number.to_string(),
            issued_amount,
            due_date: Utc::now().date_naive() + chrono::Duration::days(due_in_days),
        },
    )
    .await
    .unwrap()
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
