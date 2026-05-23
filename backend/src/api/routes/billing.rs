use axum::{
    routing::{get, post},
    Router,
};

use crate::api::{
    handlers::billing::{
        account::{
            assign_billing_account::assign_billing_account_handler,
            create_billing_account::create_billing_account_handler,
        },
        deposit::{
            apply_deposit_to_receivable::apply_deposit_to_receivable_handler,
            create_deposit::create_deposit_handler,
            reverse_deposit_application::reverse_deposit_application_handler,
        },
        folio::{
            close_folio::close_folio_handler,
            create_folio_entry::create_folio_entry_handler,
            folio_audit::get_folio_audit_handler,
            folio_query::get_folio_handler,
            open_reservation_folio::open_reservation_folio_handler,
        },
        invoice::{
            create_invoice::create_invoice_handler,
            invoice_query::{
                get_invoice_handler,
                list_billing_account_invoices_handler,
            },
            void_invoice::void_invoice_handler,
        },
        payment::{
            allocate_existing_payment::allocate_existing_payment_handler,
            create_payment::create_payment_handler,
            refund_payment::refund_payment_handler,
            reverse_payment_allocation::reverse_payment_allocation_handler,
        },
        receivable::{
            allocate_receivable_payment::allocate_receivable_payment_handler,
            receivable_detail::get_receivable_handler,
            receivable_query::get_receivable_aging_handler,
            update_receivable_status::{
                dispute_receivable_handler,
                resolve_receivable_dispute_handler,
                write_off_receivable_handler,
            },
        },
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/reservations/:id/folios",
            post(open_reservation_folio_handler),
        )
        .route("/folios/entries", post(create_folio_entry_handler))
        .route("/folios/payments", post(create_payment_handler))
        .route("/folios/deposits", post(create_deposit_handler))
        .route("/folios/:id", get(get_folio_handler))
        .route("/folios/:id/close", post(close_folio_handler))
        .route("/billing-accounts", post(create_billing_account_handler))
        .route(
            "/folios/:id/billing-account",
            post(assign_billing_account_handler),
        )
        .route("/receivables/aging", get(get_receivable_aging_handler))
        .route("/receivables/:id", get(get_receivable_handler))
        .route(
            "/receivables/:id/payments",
            post(allocate_receivable_payment_handler),
        )
        .route("/receivables/:id/dispute", post(dispute_receivable_handler))
        .route(
            "/receivables/:id/resolve-dispute",
            post(resolve_receivable_dispute_handler),
        )
        .route(
            "/receivables/:id/write-off",
            post(write_off_receivable_handler),
        )
        .route(
            "/payment-allocations/:id/reverse",
            post(reverse_payment_allocation_handler),
        )
        .route(
            "/deposit-applications/:id/reverse",
            post(reverse_deposit_application_handler),
        )
        .route("/invoices", post(create_invoice_handler))
        .route("/invoices/:id", get(get_invoice_handler))
        .route("/invoices/:id/void", post(void_invoice_handler))
        .route(
            "/billing-accounts/:id/invoices",
            get(list_billing_account_invoices_handler),
        )
        .route("/folios/:folio_id/audit", get(get_folio_audit_handler))
        .route(
            "/payments/:id/allocations",
            post(allocate_existing_payment_handler),
        )
        .route(
            "/payments/:payment_id/refunds",
            post(refund_payment_handler),
        )
        .route(
            "/deposits/:id/applications",
            post(apply_deposit_to_receivable_handler),
        )
}