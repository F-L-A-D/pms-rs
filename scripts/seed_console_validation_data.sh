#!/usr/bin/env bash
set -euo pipefail

API_BASE_URL="${API_BASE_URL:-http://localhost:3000}"

extract_id() {
  sed -n 's/.*"id":"\([^"]*\)".*/\1/p'
}

extract_folio_id() {
  sed -n 's/.*"folio_id":"\([^"]*\)".*/\1/p'
}

create_guest() {
  local last_name="$1"
  local first_name="$2"
  local email="$3"

  curl -sS -X POST "${API_BASE_URL}/guests" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"last_name\": \"${last_name}\",
      \"first_name\": \"${first_name}\",
      \"phone\": null,
      \"email\": \"${email}\",
      \"nationality\": \"JP\",
      \"birth_date\": null,
      \"gender\": null,
      \"membership_code\": null,
      \"marketing_opt_in\": false
    }"
}

create_room() {
  local room_no="$1"
  local room_class="$2"
  local capacity="$3"
  local area_sqm="$4"

  curl -sS -X POST "${API_BASE_URL}/rooms" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"room_no\": \"${room_no}\",
      \"room_class\": \"${room_class}\",
      \"capacity\": ${capacity},
      \"area_sqm\": \"${area_sqm}\",
      \"is_physical\": true
    }"
}

create_room_and_extract_id() {
  local room_no="$1"
  local room_class="$2"
  local capacity="$3"
  local area_sqm="$4"

  echo "Creating room: ${room_no} (${room_class})" >&2

  local room_response
  room_response="$(create_room "${room_no}" "${room_class}" "${capacity}" "${area_sqm}")"

  local room_id
  room_id="$(echo "${room_response}" | extract_id)"

  if [[ -z "${room_id}" ]]; then
    echo "Failed to extract room id" >&2
    echo "${room_response}" >&2
    exit 1
  fi

  echo "${room_id}"
}

create_reservation() {
  local external_id="$1"
  local check_in="$2"
  local check_out="$3"
  local room_class="$4"
  local guest_id="$5"

  curl -sS -X POST "${API_BASE_URL}/reservations" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"external_id\": \"${external_id}\",
      \"check_in\": \"${check_in}\",
      \"check_out\": \"${check_out}\",
      \"room_class\": \"${room_class}\",
      \"booking_channel\": \"direct\",
      \"source_channel\": \"manual\",
      \"plan_code\": \"console_validation\",
      \"participants\": [
        {
          \"guest_id\": \"${guest_id}\",
          \"relation_type\": \"primary\"
        }
      ],
      \"daily_details\": [],
      \"package_breakdowns\": []
    }"
}

create_guest_and_reservation() {
  local last_name="$1"
  local first_name="$2"
  local email="$3"
  local external_id="$4"
  local check_in="$5"
  local check_out="$6"
  local room_class="$7"

  echo "Creating guest: ${last_name} ${first_name}" >&2

  local guest_response
  guest_response="$(create_guest "${last_name}" "${first_name}" "${email}")"

  local guest_id
  guest_id="$(echo "${guest_response}" | extract_id)"

  if [[ -z "${guest_id}" ]]; then
    echo "Failed to extract guest id" >&2
    echo "${guest_response}" >&2
    exit 1
  fi

  echo "Creating reservation: ${external_id}" >&2

  local reservation_response
  reservation_response="$(
    create_reservation \
      "${external_id}" \
      "${check_in}" \
      "${check_out}" \
      "${room_class}" \
      "${guest_id}"
  )"

  local reservation_id
  reservation_id="$(echo "${reservation_response}" | extract_id)"

  if [[ -z "${reservation_id}" ]]; then
    echo "Failed to extract reservation id" >&2
    echo "${reservation_response}" >&2
    exit 1
  fi

  echo "${reservation_id}"
}

get_reservation() {
  local reservation_id="$1"

  curl -sS -X GET "${API_BASE_URL}/reservations/${reservation_id}" \
    -H "Accept: application/json"
}

get_folio_id_for_reservation() {
  local reservation_id="$1"

  local reservation_response
  reservation_response="$(get_reservation "${reservation_id}")"

  local folio_id
  folio_id="$(echo "${reservation_response}" | extract_folio_id)"

  if [[ -z "${folio_id}" ]]; then
    echo "Failed to extract folio id for reservation ${reservation_id}" >&2
    echo "${reservation_response}" >&2
    exit 1
  fi

  echo "${folio_id}"
}

patch_reservation() {
  local reservation_id="$1"
  local body="$2"

  curl -sS -X PATCH "${API_BASE_URL}/reservations/${reservation_id}" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "${body}"
}

cancel_reservation() {
  local reservation_id="$1"

  curl -sS -X DELETE "${API_BASE_URL}/reservations/${reservation_id}" \
    -H "Accept: application/json"
}

mark_no_show() {
  local reservation_id="$1"

  curl -sS -X POST "${API_BASE_URL}/reservations/${reservation_id}/no-show" \
    -H "Accept: application/json"
}

reinstate_reservation() {
  local reservation_id="$1"

  curl -sS -X POST "${API_BASE_URL}/reservations/${reservation_id}/reinstate" \
    -H "Accept: application/json"
}

assign_room() {
  local reservation_id="$1"
  local room_id="$2"

  curl -sS -X POST "${API_BASE_URL}/reservations/${reservation_id}/assign-room/${room_id}" \
    -H "Accept: application/json"
}

create_note() {
  local reservation_id="$1"
  local kind="$2"
  local department_code="$3"
  local body="$4"

  curl -sS -X POST "${API_BASE_URL}/reservations/${reservation_id}/notes" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"kind\": \"${kind}\",
      \"department_code\": \"${department_code}\",
      \"body\": \"${body}\",
      \"actor_id\": \"console-seed\"
    }"
}

create_trace() {
  local reservation_id="$1"
  local department_code="$2"
  local body="$3"

  curl -sS -X POST "${API_BASE_URL}/reservations/${reservation_id}/traces" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"department_code\": \"${department_code}\",
      \"body\": \"${body}\",
      \"actor_id\": \"console-seed\"
    }"
}

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

  curl -sS \
    -X POST "${API_BASE_URL}/folios/deposits" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"folio_id\": \"${folio_id}\",
      \"amount\": \"${amount}\",
      \"method\": \"${method}\",
      \"external_reference\": \"${external_reference}\"
    }"
}

create_payment() {
  local folio_id="$1"
  local amount="$2"
  local method="$3"
  local external_reference="$4"

  curl -sS \
    -X POST "${API_BASE_URL}/folios/payments" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -d "{
      \"folio_id\": \"${folio_id}\",
      \"amount\": \"${amount}\",
      \"method\": \"${method}\",
      \"external_reference\": \"${external_reference}\"
    }"
}

echo "Seeding console validation data..." >&2

echo "Creating rooms..." >&2

standard_room_101_id="$(create_room_and_extract_id "101" "standard" "2" "18.5")"
standard_room_102_id="$(create_room_and_extract_id "102" "standard" "2" "18.5")"
deluxe_room_301_id="$(create_room_and_extract_id "301" "deluxe" "4" "32.0")"
suite_room_801_id="$(create_room_and_extract_id "801" "suite" "4" "55.0")"

echo "Creating reservations..." >&2

confirmed_id="$(
  create_guest_and_reservation \
    "Demo" \
    "Console" \
    "console.demo@example.com" \
    "console-seed-confirmed-001" \
    "2026-06-01" \
    "2026-06-03" \
    "standard"
)"

modified_id="$(
  create_guest_and_reservation \
    "Modify" \
    "Console" \
    "console.modify@example.com" \
    "console-seed-modified-001" \
    "2026-06-04" \
    "2026-06-06" \
    "standard"
)"

cancelled_id="$(
  create_guest_and_reservation \
    "Cancel" \
    "Console" \
    "console.cancel@example.com" \
    "console-seed-cancelled-001" \
    "2026-06-07" \
    "2026-06-09" \
    "superior"
)"

no_show_id="$(
  create_guest_and_reservation \
    "NoShow" \
    "Console" \
    "console.noshow@example.com" \
    "console-seed-no-show-001" \
    "2026-06-10" \
    "2026-06-12" \
    "deluxe"
)"

reinstated_id="$(
  create_guest_and_reservation \
    "Reinstate" \
    "Console" \
    "console.reinstate@example.com" \
    "console-seed-reinstate-001" \
    "2026-06-13" \
    "2026-06-15" \
    "suite"
)"

range_id="$(
  create_guest_and_reservation \
    "Range" \
    "Tester" \
    "range.tester@example.com" \
    "console-seed-range-001" \
    "2026-07-01" \
    "2026-07-05" \
    "standard"
)"

past_id="$(
  create_guest_and_reservation \
    "Past" \
    "Completed" \
    "past.completed@example.com" \
    "console-seed-past-001" \
    "2026-05-01" \
    "2026-05-03" \
    "standard"
)"

echo "Assigning rooms..." >&2

assign_room "${confirmed_id}" "${standard_room_101_id}" >/dev/null
assign_room "${no_show_id}" "${deluxe_room_301_id}" >/dev/null
assign_room "${range_id}" "${standard_room_102_id}" >/dev/null

echo "Adding note to confirmed reservation" >&2
create_note \
  "${confirmed_id}" \
  "internal" \
  "front" \
  "Console seed note: VIP guest. Confirm arrival time." >/dev/null

echo "Adding trace to confirmed reservation" >&2
create_trace \
  "${confirmed_id}" \
  "front" \
  "Console seed trace: guest requested late check-in." >/dev/null

echo "Modifying reservation" >&2
patch_reservation \
  "${modified_id}" \
  "{
    \"expected_version\": 1,
    \"room_class\": \"suite\"
  }" >/dev/null

echo "Assigning room to modified reservation after room_class change" >&2
assign_room "${modified_id}" "${suite_room_801_id}" >/dev/null

echo "Cancelling reservation" >&2
cancel_reservation "${cancelled_id}" >/dev/null

echo "Marking no-show reservation" >&2
mark_no_show "${no_show_id}" >/dev/null

echo "Cancelling and reinstating reservation" >&2
cancel_reservation "${reinstated_id}" >/dev/null
reinstate_reservation "${reinstated_id}" >/dev/null

echo "Adding note to range reservation" >&2
create_note \
  "${range_id}" \
  "internal" \
  "front" \
  "Console seed note: range-search target." >/dev/null

echo "Adding trace to past reservation" >&2
create_trace \
  "${past_id}" \
  "front" \
  "Console seed trace: past reservation validation." >/dev/null

echo "Adding billing validation data..." >&2

confirmed_folio_id="$(get_folio_id_for_reservation "${confirmed_id}")"
range_folio_id="$(get_folio_id_for_reservation "${range_id}")"
past_folio_id="$(get_folio_id_for_reservation "${past_id}")"

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

echo "Adding past reservation billing data" >&2
create_folio_entry \
  "${past_folio_id}" \
  "room_charge" \
  "8000" \
  "Console seed: past reservation room charge" >/dev/null

create_folio_entry \
  "${past_folio_id}" \
  "manual_adjustment" \
  "-1000" \
  "Console seed: goodwill adjustment" >/dev/null

create_payment \
  "${past_folio_id}" \
  "7000" \
  "cash" \
  "console-seed-payment-past-001" >/dev/null

echo "Billing validation expectations:" >&2
echo "- confirmed: charges=13200 payments=5000 balance=8200" >&2
echo "- range: charges=20000 payments=20000 balance=0" >&2
echo "- past: charges=8000 payments=8000 balance=0" >&2

echo "Done." >&2