# Current State

## Phase

Projection runtime stabilization and hospitality operational expansion have reached a stable backend foundation.

The current focus has shifted from backend-only semantic expansion to an integrated operational console phase:

- backend/frontend repository separation
- frontend console foundation
- backend health check connectivity
- Reservation Search as the first operational discovery surface
- Reservation Detail as the first vertical UI slice
- Folio Detail as the first Billing workflow slice
- practical discovery of missing workflow, DTO, projection, audit, and linked-resource requirements through UI usage

The goal is not frontend completeness. The goal is to use a minimal console UI to expose operational gaps that are difficult to find from backend tests alone.

Current reservation and billing console validation has reached a stable checkpoint:

- Reservation Search MVP is implemented.
- Reservation Detail workflow validation is implemented.
- Folio Detail workflow validation is implemented.
- Reservation ↔ Folio linkage is verified.
- `linked_resources.folio_id` is visible from both Search and Detail.
- Reservation Detail → Folio Detail navigation is implemented and verified.
- Next focus is Billing Audit Workflow Validation.

---

## Folio Detail Validation

Folio Detail workflow validation is implemented.

Current Folio Detail verifies visibility for:

- folio summary
- reservation linkage
- folio status
- billing_account_id
- folio entries
- room charges
- tax charges
- deposits
- payments
- manual adjustments
- derived charges total
- derived payments total
- derived balance

Implemented:

```text
Reservation Detail
    ↓
Folio Detail
```

Navigation is verified.

Current Folio Detail response includes:

```text
folio
entries
total_charges
total_payments
balance
```

Validation completed:

```text
RoomCharge
TaxCharge
DepositReceived
PaymentApplied
ManualAdjustment
```

Verified scenarios:

confirmed reservation

```text
charges 13200
payments 5000
balance 8200
```

range reservation

```text
charges 20000
payments 20000
balance 0
```

past reservation

```text
room charge
manual adjustment
payment applied
balance 0
```

Verified:

- reservation → folio navigation
- folio detail API
- folio detail UI
- folio entries visibility
- derived balance calculation
- deposit visibility
- payment visibility
- room charge visibility

Current gaps identified:

- payment detail visibility
- payment method visibility
- payment reference visibility
- folio audit visibility
- folio operation event visibility
- invoice linkage visibility

---

## Seed / Console Validation Data

Console validation seed scenarios are available for:

- confirmed
- modified
- cancelled
- no_show
- reinstated
- range search
- room assigned
- room unassigned
- note
- trace
- room charge
- tax charge
- deposit
- payment
- manual adjustment
- balance validation

These seeds are used to validate Search, Detail, room assignment lifecycle, audit visibility, operation event visibility, linked-resource visibility, billing visibility, and balance calculation.

---

## First UI Slice Status

The first meaningful frontend slice was Reservation Search → Reservation Detail.

This slice validated that backend/API/projection semantics are sufficient for reservation workflow inspection.

The second meaningful frontend slice is Reservation Detail → Folio Detail.

This slice validated that billing workflows are inspectable through the console and that balance derivation behaves correctly from operational folio entries.

Confirmed visible from the UI:

- reservation summary
- booking_channel
- source_channel
- room assignment status
- participants
- participant details
- linked resources
- folio_id
- notes
- traces
- audit logs
- operation events
- folio entries
- room charges
- tax charges
- deposits
- payments
- balance

The purpose remains discovery of missing backend capability, not final UI design.

---

## Current Expansion Areas

Good next areas:

- Billing audit log visibility
- Billing operation event visibility
- Payment detail visibility
- Payment method visibility
- Payment reference visibility
- Invoice linkage visibility
- Guest Detail after Folio Detail
- Reservation Detail → Guest Detail navigation

Areas that should remain stable:

- projection runtime and topology orchestration
- transaction ownership rules
- repository transaction behavior
- explicit enum snake_case persistence
- operational/projection authority separation
- backend as semantic authority

---

## Next Phase

Next phase:

```text
Billing Audit Workflow Validation
```

Primary objective:

Use Folio Detail as an operational inspection surface to validate billing traceability.

Focus areas:

- audit logs
- operation events
- payment traceability
- actor visibility
- operation source visibility
- payment method visibility
- payment reference visibility
- invoice linkage

The objective is to answer:

```text
Why did this balance occur?
Who created the entry?
When was it created?
Which workflow created it?
```

The goal remains discovery of missing:

- workflow
- business logic
- navigation
- events
- DTO fields
- query models
- projections
- operational data

Do not over-polish the UI.