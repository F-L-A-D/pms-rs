#!/usr/bin/env bash

echo "Adding billing validation data..." >&2

confirmed_folio_id="$(get_folio_id_for_reservation "${confirmed_id}")"
range_folio_id="$(get_folio_id_for_reservation "${range_id}")"
settled_folio_id="$(get_folio_id_for_reservation "${settled_id}")"

echo "Adding folio entries to confirmed reservation" >&2
create_folio_entry \
  "${confirmed_folio_id}" \
  "room_charge" \
  "12000" \
  "Console seed: room charge" >/dev/null

create_folio_entry \
  "${confirmed_folio_id}" \
  "tax_charge" \
  "1200" \
  "Console seed: accommodation tax" >/dev/null

create_deposit \
  "${confirmed_folio_id}" \
  "5000" \
  "cash" \
  "console-seed-deposit-001" >/dev/null

echo "Adding settled folio data to range reservation" >&2
create_folio_entry \
  "${range_folio_id}" \
  "room_charge" \
  "20000" \
  "Console seed: range reservation room charge" >/dev/null

create_payment \
  "${range_folio_id}" \
  "20000" \
  "credit_card" \
  "console-seed-payment-001" >/dev/null

echo "Adding settled reservation billing data" >&2
create_folio_entry \
  "${settled_folio_id}" \
  "room_charge" \
  "8000" \
  "Console seed: settled reservation room charge" >/dev/null

create_folio_entry \
  "${settled_folio_id}" \
  "manual_adjustment" \
  "-1000" \
  "Console seed: goodwill adjustment" >/dev/null

create_payment \
  "${settled_folio_id}" \
  "7000" \
  "cash" \
  "console-seed-payment-settled-001" >/dev/null

echo "Creating billing account for range folio" >&2

billing_account_response="$(
  create_billing_account \
    "Console Seed Billing Account"
)"

billing_account_id="$(echo "${billing_account_response}" | extract_id)"

if [[ -z "${billing_account_id}" ]]; then
  echo "Failed to extract billing account id" >&2
  echo "${billing_account_response}" >&2
  exit 1
fi

echo "Assigning billing account to range folio" >&2

assign_billing_account \
  "${range_folio_id}" \
  "${billing_account_id}" >/dev/null

echo "Closing range folio before invoice issue" >&2

close_folio "${range_folio_id}" >/dev/null

echo "Adding invoice audit validation data to range reservation" >&2

invoice_response="$(
  create_invoice \
    "${range_folio_id}" \
    "INV-CONSOLE-SEED-001" \
    "20000" \
    "2026-07-10"
)"

invoice_id="$(echo "${invoice_response}" | extract_id)"

if [[ -z "${invoice_id}" ]]; then
  echo "Failed to extract invoice id" >&2
  echo "${invoice_response}" >&2
  exit 1
fi

void_invoice \
  "${invoice_id}" \
  "Console seed: invoice void audit validation" >/dev/null

echo "Billing validation expectations:" >&2
echo "- confirmed: charges=13200 payments=5000 balance=8200" >&2
echo "- range: charges=20000 payments=20000 balance=0" >&2
echo "- settled: charges=7000 payments=7000 balance=0" >&2
echo "- confirmed audit: receive_deposit" >&2
echo "- range audit: apply_payment + close_folio + issue_invoice + void_invoice" >&2
echo "- settled audit: apply_payment" >&2
