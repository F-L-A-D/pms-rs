#!/usr/bin/env bash

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