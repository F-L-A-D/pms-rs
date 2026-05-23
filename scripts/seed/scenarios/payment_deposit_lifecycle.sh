#!/usr/bin/env bash

echo "Adding payment/deposit lifecycle validation data..." >&2

payment_deposit_reservation_id="$(
  create_guest_and_reservation \
    "PaymentDeposit" \
    "Tester" \
    "payment.deposit.tester@example.com" \
    "console-seed-payment-deposit-001" \
    "2026-07-11" \
    "2026-07-12" \
    "standard"
)"

payment_deposit_folio_id="$(
  get_folio_id_for_reservation "${payment_deposit_reservation_id}"
)"

echo "Receiving unapplied payment..." >&2

payment_response="$(
  create_payment \
    "${payment_deposit_folio_id}" \
    "3000" \
    "credit_card" \
    "console-seed-payment-001" \
    "Console seed: receive unapplied payment validation"
)"

payment_id="$(
  echo "${payment_response}" | extract_id
)"

if [[ -z "${payment_id}" ]]; then
  echo "Failed to extract payment id" >&2
  echo "${payment_response}" >&2
  exit 1
fi

echo "Receiving deposit..." >&2

deposit_response="$(
  create_deposit \
    "${payment_deposit_folio_id}" \
    "5000" \
    "credit_card" \
    "console-seed-deposit-001" \
    "Console seed: receive deposit validation"
)"

deposit_id="$(
  echo "${deposit_response}" | extract_id
)"

if [[ -z "${deposit_id}" ]]; then
  echo "Failed to extract deposit id" >&2
  echo "${deposit_response}" >&2
  exit 1
fi

echo "Payment/Deposit lifecycle validation expectations:" >&2
echo "- payment audit: receive_payment / payment.receive" >&2
echo "- payment status: unapplied" >&2
echo "- payment unapplied_amount: 3000" >&2
echo "- deposit audit: receive_deposit / deposit.receive" >&2
echo "- deposit status: held" >&2
echo "- deposit unapplied_amount: 5000" >&2

echo "Adding receivable for existing payment allocation..." >&2

create_folio_entry \
  "${payment_deposit_folio_id}" \
  "room_charge" \
  "3000" \
  "Console seed: existing payment allocation room charge" >/dev/null

payment_deposit_billing_account_response="$(
  create_billing_account \
    "Console Seed Payment Deposit Account"
)"

payment_deposit_billing_account_id="$(
  echo "${payment_deposit_billing_account_response}" | extract_id
)"

if [[ -z "${payment_deposit_billing_account_id}" ]]; then
  echo "Failed to extract payment deposit billing account id" >&2
  echo "${payment_deposit_billing_account_response}" >&2
  exit 1
fi

assign_billing_account \
  "${payment_deposit_folio_id}" \
  "${payment_deposit_billing_account_id}" >/dev/null

close_folio "${payment_deposit_folio_id}" >/dev/null

payment_deposit_invoice_response="$(
  create_invoice \
    "${payment_deposit_folio_id}" \
    "INV-CONSOLE-SEED-PAYMENT-DEPOSIT-001" \
    "3000" \
    "2026-07-20"
)"

payment_deposit_invoice_id="$(
  echo "${payment_deposit_invoice_response}" | extract_id
)"

if [[ -z "${payment_deposit_invoice_id}" ]]; then
  echo "Failed to extract payment deposit invoice id" >&2
  echo "${payment_deposit_invoice_response}" >&2
  exit 1
fi

payment_deposit_invoice_detail="$(
  get_invoice "${payment_deposit_invoice_id}"
)"

payment_deposit_receivable_id="$(
  echo "${payment_deposit_invoice_detail}" | extract_receivable_id
)"

if [[ -z "${payment_deposit_receivable_id}" ]]; then
  echo "Failed to extract payment deposit receivable id" >&2
  echo "${payment_deposit_invoice_detail}" >&2
  exit 1
fi

echo "Allocating existing payment to receivable..." >&2

existing_payment_allocation_response="$(
  allocate_existing_payment \
    "${payment_id}" \
    "${payment_deposit_receivable_id}" \
    "3000" \
    "Console seed: existing payment allocation validation"
)"

existing_payment_allocation_id="$(
  echo "${existing_payment_allocation_response}" | extract_allocation_id
)"

if [[ -z "${existing_payment_allocation_id}" ]]; then
  echo "Failed to extract existing payment allocation id" >&2
  echo "${existing_payment_allocation_response}" >&2
  exit 1
fi

echo "- existing payment allocation audit: payment.allocate" >&2
echo "- payment status after allocation: applied" >&2
echo "- payment unapplied_amount after allocation: 0" >&2
echo "- receivable status after allocation: settled" >&2