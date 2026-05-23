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