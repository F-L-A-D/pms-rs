#!/usr/bin/env bash

create_folio_entry() {
  local folio_id="$1"
  local entry_type="$2"
  local amount="$3"
  local memo="$4"

  curl -sS \
    -X POST "${API_BASE_URL}/folios/entries" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"folio_id\": \"${folio_id}\",
      \"entry_type\": \"${entry_type}\",
      \"amount\": \"${amount}\",
      \"memo\": \"${memo}\"
    }"
}

create_deposit() {
  local folio_id="$1"
  local amount="$2"
  local method="$3"
  local external_reference="$4"
  local reason="${5:-}"

  curl -sS \
    -X POST "${API_BASE_URL}/folios/deposits" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"folio_id\": \"${folio_id}\",
      \"amount\": \"${amount}\",
      \"method\": \"${method}\",
      \"external_reference\": \"${external_reference}\",
      \"reason\": \"${reason}\"
    }"
}

create_payment() {
  local folio_id="$1"
  local amount="$2"
  local method="$3"
  local external_reference="$4"
  local reason="${5:-}"

  curl -sS \
    -X POST "${API_BASE_URL}/folios/payments" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"folio_id\": \"${folio_id}\",
      \"amount\": \"${amount}\",
      \"method\": \"${method}\",
      \"external_reference\": \"${external_reference}\",
      \"reason\": \"${reason}\"
    }"
}

create_billing_account() {
  local name="$1"

  curl -sS \
    -X POST "${API_BASE_URL}/billing-accounts" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"company_id\": null,
      \"name\": \"${name}\"
    }"
}

assign_billing_account() {
  local folio_id="$1"
  local billing_account_id="$2"

  curl -sS \
    -X POST "${API_BASE_URL}/folios/${folio_id}/billing-account" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"billing_account_id\": \"${billing_account_id}\"
    }"
}

close_folio() {
  local folio_id="$1"

  curl -sS \
    -X POST "${API_BASE_URL}/folios/${folio_id}/close" \
    -H "Accept: application/json"
}

create_invoice() {
  local folio_id="$1"
  local invoice_number="$2"
  local issued_amount="$3"
  local due_date="$4"

  curl -sS \
    -X POST "${API_BASE_URL}/invoices" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"folio_id\": \"${folio_id}\",
      \"invoice_number\": \"${invoice_number}\",
      \"issued_amount\": \"${issued_amount}\",
      \"due_date\": \"${due_date}\"
    }"
}

void_invoice() {
  local invoice_id="$1"
  local reason="$2"

  curl -sS \
    -X POST "${API_BASE_URL}/invoices/${invoice_id}/void" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"reason\": \"${reason}\"
    }"
}

get_invoice() {
  local invoice_id="$1"

  curl -sS \
    -X GET "${API_BASE_URL}/invoices/${invoice_id}" \
    -H "Accept: application/json"
}

dispute_receivable() {
  local receivable_id="$1"
  local reason="$2"

  curl -sS \
    -X POST \
    "${API_BASE_URL}/receivables/${receivable_id}/dispute" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"reason\": \"${reason}\"
    }"
}

resolve_receivable_dispute() {
  local receivable_id="$1"
  local reason="$2"

  curl -sS \
    -X POST \
    "${API_BASE_URL}/receivables/${receivable_id}/resolve-dispute" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"reason\": \"${reason}\"
    }"
}

allocate_receivable_payment() {
  local receivable_id="$1"
  local amount="$2"
  local method="$3"
  local external_reference="$4"
  local reason="$5"

  curl -sS \
    -X POST \
    "${API_BASE_URL}/receivables/${receivable_id}/payments" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"amount\": \"${amount}\",
      \"method\": \"${method}\",
      \"external_reference\": \"${external_reference}\",
      \"reason\": \"${reason}\"
    }"
}

reverse_payment_allocation() {
  local allocation_id="$1"
  local reason="$2"

  curl -sS \
    -X POST \
    "${API_BASE_URL}/payment-allocations/${allocation_id}/reverse" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"reason\": \"${reason}\"
    }"
}

write_off_receivable() {
  local receivable_id="$1"
  local reason="$2"

  curl -sS -X POST \
    "${API_BASE_URL}/receivables/${receivable_id}/write-off" \
    -H "Content-Type: application/json" \
    -d "{
      \"reason\": \"${reason}\"
    }"
}