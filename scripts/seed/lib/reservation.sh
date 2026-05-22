#!/usr/bin/env bash

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