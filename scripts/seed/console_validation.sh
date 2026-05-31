#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(
  cd "$(dirname "${BASH_SOURCE[0]}")" &&
  pwd
)"

export API_BASE_URL="${API_BASE_URL:-http://localhost:3000}"

source "${SCRIPT_DIR}/lib/common.sh"
source "${SCRIPT_DIR}/lib/guest.sh"
source "${SCRIPT_DIR}/lib/room.sh"
source "${SCRIPT_DIR}/lib/reservation.sh"
source "${SCRIPT_DIR}/lib/billing.sh"

echo "Seeding console validation data..." >&2
echo "Current business date:" >&2
curl -sS -X GET "${API_BASE_URL}/business-date/current" \
  -H "Accept: application/json" >&2
echo >&2

source "${SCRIPT_DIR}/scenarios/reservation_workflow.sh"
source "${SCRIPT_DIR}/scenarios/billing_audit.sh"
source "${SCRIPT_DIR}/scenarios/billing_receivable.sh"
source "${SCRIPT_DIR}/scenarios/payment_deposit_lifecycle.sh"

echo "Done." >&2
