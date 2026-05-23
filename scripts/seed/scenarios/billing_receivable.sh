#!/usr/bin/env bash

echo "Adding receivable audit validation data..." >&2

receivable_reservation_id="$(
  create_guest_and_reservation \
    "Receivable" \
    "Tester" \
    "receivable.tester@example.com" \
    "console-seed-receivable-001" \
    "2026-07-06" \
    "2026-07-08" \
    "standard"
)"

receivable_folio_id="$(
  get_folio_id_for_reservation "${receivable_reservation_id}"
)"

create_folio_entry \
  "${receivable_folio_id}" \
  "room_charge" \
  "5000" \
  "Console seed: receivable dispute room charge" >/dev/null

receivable_billing_account_response="$(
  create_billing_account \
    "Console Seed Receivable Account"
)"

receivable_billing_account_id="$(
  echo "${receivable_billing_account_response}" | extract_id
)"

if [[ -z "${receivable_billing_account_id}" ]]; then
  echo "Failed to extract receivable billing account id" >&2
  echo "${receivable_billing_account_response}" >&2
  exit 1
fi

assign_billing_account \
  "${receivable_folio_id}" \
  "${receivable_billing_account_id}" >/dev/null

close_folio "${receivable_folio_id}" >/dev/null

receivable_invoice_response="$(
  create_invoice \
    "${receivable_folio_id}" \
    "INV-CONSOLE-SEED-DISPUTE-001" \
    "5000" \
    "2026-07-15"
)"

receivable_invoice_id="$(
  echo "${receivable_invoice_response}" | extract_id
)"

if [[ -z "${receivable_invoice_id}" ]]; then
  echo "Failed to extract invoice id" >&2
  echo "${receivable_invoice_response}" >&2
  exit 1
fi

receivable_detail="$(
  get_invoice "${receivable_invoice_id}"
)"

receivable_id="$(
  echo "${receivable_detail}" | extract_receivable_id
)"

if [[ -z "${receivable_id}" ]]; then
  echo "Failed to extract receivable id" >&2
  echo "${receivable_detail}" >&2
  exit 1
fi

echo "Allocating receivable payment..." >&2

allocation_response="$(
  allocate_receivable_payment \
    "${receivable_id}" \
    "3000" \
    "credit_card" \
    "console-seed-allocation-001" \
    "Console seed: allocation validation"
)"

allocation_id="$(
  echo "${allocation_response}" | extract_allocation_id
)"

if [[ -z "${allocation_id}" ]]; then
  echo "Failed to extract allocation id" >&2
  echo "${allocation_response}" >&2
  exit 1
fi

echo "Reversing payment allocation..." >&2

reverse_payment_allocation \
  "${allocation_id}" \
  "Console seed: reverse allocation validation" >/dev/null

echo "Disputing receivable..." >&2

dispute_receivable \
  "${receivable_id}" \
  "Console seed: dispute validation" >/dev/null

echo "Resolving receivable dispute..." >&2

resolve_receivable_dispute \
  "${receivable_id}" \
  "Console seed: dispute resolved validation" >/dev/null

echo "Receivable validation expectations:" >&2
echo "- receivable audit: issue_invoice + allocate_receivable_payment + reverse_payment_allocation + dispute_receivable + resolve_receivable_dispute" >&2
echo "- dispute: open -> disputed" >&2
echo "- resolve: disputed -> open" >&2

echo "Adding write-off receivable validation data..." >&2

write_off_reservation_id="$(
  create_guest_and_reservation \
    "WriteOff" \
    "Tester" \
    "writeoff.tester@example.com" \
    "console-seed-writeoff-001" \
    "2026-07-09" \
    "2026-07-10" \
    "standard"
)"

write_off_folio_id="$(
  get_folio_id_for_reservation "${write_off_reservation_id}"
)"

create_folio_entry \
  "${write_off_folio_id}" \
  "room_charge" \
  "5000" \
  "Console seed: write-off room charge" >/dev/null

write_off_billing_account_response="$(
  create_billing_account \
    "Console Seed WriteOff Account"
)"

write_off_billing_account_id="$(
  echo "${write_off_billing_account_response}" | extract_id
)"

if [[ -z "${write_off_billing_account_id}" ]]; then
  echo "Failed to extract write-off billing account id" >&2
  echo "${write_off_billing_account_response}" >&2
  exit 1
fi

assign_billing_account \
  "${write_off_folio_id}" \
  "${write_off_billing_account_id}" >/dev/null

close_folio "${write_off_folio_id}" >/dev/null

write_off_invoice_response="$(
  create_invoice \
    "${write_off_folio_id}" \
    "INV-CONSOLE-SEED-WRITEOFF-001" \
    "5000" \
    "2026-07-16"
)"

write_off_invoice_id="$(
  echo "${write_off_invoice_response}" | extract_id
)"

if [[ -z "${write_off_invoice_id}" ]]; then
  echo "Failed to extract write-off invoice id" >&2
  echo "${write_off_invoice_response}" >&2
  exit 1
fi

write_off_receivable_detail="$(
  get_invoice "${write_off_invoice_id}"
)"

write_off_receivable_id="$(
  echo "${write_off_receivable_detail}" | extract_receivable_id
)"

if [[ -z "${write_off_receivable_id}" ]]; then
  echo "Failed to extract write-off receivable id" >&2
  echo "${write_off_receivable_detail}" >&2
  exit 1
fi

echo "Writing off receivable..." >&2

write_off_receivable \
  "${write_off_receivable_id}" \
  "Console seed: write-off validation" >/dev/null

echo "Write-off validation expectations:" >&2
echo "- receivable audit: issue_invoice + write_off_receivable" >&2
echo "- write_off: open -> written_off" >&2
echo "- outstanding_amount: 5000 -> 0" >&2