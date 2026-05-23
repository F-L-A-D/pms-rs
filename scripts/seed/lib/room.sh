#!/usr/bin/env bash

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