#!/usr/bin/env bash

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

export confirmed_id
export range_id
export past_id

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