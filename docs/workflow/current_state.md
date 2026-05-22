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
- Billing Audit as the first billing traceability validation surface
- practical discovery of missing workflow, DTO, projection, audit, and linked-resource requirements through UI usage

The goal is not frontend completeness. The goal is to use a minimal console UI to expose operational gaps that are difficult to find from backend tests alone.

Current reservation and billing console validation has reached a stable checkpoint:

- Reservation Search MVP is implemented.
- Reservation Detail workflow validation is implemented.
- Folio Detail workflow validation is implemented.
- Billing Audit workflow validation is implemented at console-validation level.
- Reservation ↔ Folio linkage is verified.
- `linked_resources.folio_id` remains the Open / Locked representative folio link for backward compatibility.
- Reservation Detail now exposes all linked folios through `folios`.
- Closed folios are reachable from Reservation Detail.
- Reservation Detail → Folio Detail navigation is implemented and verified.
- Folio Detail → Billing Audit visibility is implemented and verified.

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

Implemented flow:

Reservation Detail
→ Folio Detail

Navigation is verified.

Current Folio Detail response includes:

- folio
- entries
- total_charges
- total_payments
- balance

Validation completed:

- RoomCharge
- TaxCharge
- DepositReceived
- PaymentApplied
- ManualAdjustment

Verified scenarios:

confirmed reservation:

- charges 13200
- payments 5000
- balance 8200

range reservation:

- charges 20000
- payments 20000
- balance 0

past reservation:

- room charge
- manual adjustment
- payment applied
- balance 0

Verified:

- reservation → folio navigation
- folio detail API
- folio detail UI
- folio entries visibility
- derived balance calculation
- deposit visibility
- payment visibility
- room charge visibility
- closed folio navigation from Reservation Detail

---

## Billing Audit Workflow Validation

Billing Audit Workflow Validation has reached a working console checkpoint.

The console now supports the operational path:

Reservation Search
→ Reservation Detail
→ Folio Detail
→ Billing Audit

Billing Audit currently validates traceability for the main billing workflow:

payment applied
→ billing account assigned
→ folio closed
→ invoice issued
→ invoice voided

Implemented backend capabilities:

- Billing Audit API from Folio Detail.
- `BillingAuditResponse` with business-readable display fields:
  - amount
  - payment_method
  - payment_reference
  - invoice_number
  - issued_amount
  - billing_account_id
  - billing_account_name
- operation event recording for:
  - create_payment
  - create_deposit
  - create_invoice
  - void_invoice
  - assign_billing_account
  - close_folio
- billing account creation API.
- close folio handler and route.
- assign billing account request cleanup:
  - `folio_id` is sourced from Path
  - request body carries only `billing_account_id`
- billing operation types:
  - apply_payment
  - receive_deposit
  - assign_billing_account
  - close_folio
  - issue_invoice
  - void_invoice
- fixed operation type persistence typo:
  - `assigun_billing_account` → `assign_billing_account`

Implemented frontend capabilities:

- Billing Audit section on Folio Detail.
- Billing Audit table with business-readable columns:
  - Amount
  - Method
  - Reference
  - Invoice
  - Billing Account
  - Actor
  - Source
  - Reason
  - Trace
- Reservation Detail folio grouping:
  - Active Folios
  - Other Folios
- Closed folios can be reached from Reservation Detail.

Validated Billing Audit response for range reservation:

- apply_payment
- assign_billing_account
- close_folio
- issue_invoice
- void_invoice

Verified display fields:

apply_payment:

- amount
- payment_method
- payment_reference

assign_billing_account:

- billing_account_id
- billing_account_name

close_folio:

- billing_account_id
- billing_account_name
- status transition

issue_invoice:

- invoice_number
- issued_amount
- billing_account_name

void_invoice:

- invoice_number
- issued_amount
- billing_account_name

Current Billing Audit response includes:

- operation_id
- folio_id
- folio_entry_id
- payment_id
- invoice_id
- aggregate_type
- aggregate_id
- operation_type
- actor
- actor_id
- source
- action
- reason
- amount
- payment_method
- payment_reference
- invoice_number
- issued_amount
- billing_account_id
- billing_account_name
- before_json
- after_json
- changed_fields_json
- occurred_at

---

## Reservation Detail Folio Links

Reservation Detail now distinguishes between the backward-compatible active folio link and all linked folios.

Existing field:

- `linked_resources.folio_id`

Meaning:

- Open / Locked representative folio.
- Used for backward compatibility and active folio visibility.

New field:

- `folios`

Meaning:

- All folios linked to the reservation, including Closed folios.

Console display:

- Active Folios
- Other Folios

This resolves the issue where closed folios disappeared from Reservation Detail after invoice workflow validation.

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
- billing account creation
- billing account assignment
- folio close
- invoice issue
- invoice void
- billing audit traceability

These seeds are used to validate Search, Detail, room assignment lifecycle, audit visibility, operation event visibility, linked-resource visibility, billing visibility, invoice visibility, and balance calculation.

Current Billing validation seed expectations:

confirmed reservation:

- charges 13200
- payments 5000
- balance 8200
- audit: receive_deposit

range reservation:

- charges 20000
- payments 20000
- balance 0
- audit: apply_payment + assign_billing_account + close_folio + issue_invoice + void_invoice

past reservation:

- charges 8000
- payments 8000
- balance 0
- audit: apply_payment

---

## First UI Slice Status

The first meaningful frontend slice was Reservation Search → Reservation Detail.

This slice validated that backend/API/projection semantics are sufficient for reservation workflow inspection.

The second meaningful frontend slice is Reservation Detail → Folio Detail.

This slice validated that billing workflows are inspectable through the console and that balance derivation behaves correctly from operational folio entries.

The third meaningful frontend slice is Folio Detail → Billing Audit.

This slice validated that billing workflows can be traced through operation events, audit logs, and business-readable display fields.

Confirmed visible from the UI:

- reservation summary
- booking_channel
- source_channel
- room assignment status
- participants
- participant details
- linked resources
- active folio link
- all folio links
- closed folio links
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
- billing audit events
- billing account assignment
- folio close
- invoice issue
- invoice void
- payment method
- payment reference
- invoice number
- billing account name

The purpose remains discovery of missing backend capability, not final UI design.

---

## Current Expansion Areas

Good next areas:

- Billing audit reason handling
- Void invoice before/after correction
- Payment detail visibility
- Invoice detail visibility
- Payment Detail navigation
- Invoice Detail navigation
- Billing Audit read model / projection decision
- OperationContext propagation from handlers
- Guest Detail after Folio Detail
- Reservation Detail → Guest Detail navigation

Areas that should remain stable:

- projection runtime and topology orchestration
- transaction ownership rules
- repository transaction behavior
- explicit enum snake_case persistence
- operational/projection authority separation
- backend as semantic authority
- linked-resource separation between active representative link and all linked resources

---

## Current Known Gaps

### Void invoice changed fields

`void_invoice.changed_fields_json.receivable_status.before_value` is currently `null`.

Current:

- receivable_status: null → voided

Target:

- receivable_status: open → voided

### Reason handling

`reason` is currently mostly `null`.

Reason should become required or strongly encouraged for high-risk billing operations:

- void_invoice
- manual_adjustment
- refund
- write_off
- reopen_folio
- payment reversal
- receivable dispute

### Billing Audit query model

Billing Audit API currently uses operation event scan plus `after_json` extraction.

This is acceptable for validation.

Later candidates:

- `billing_audit_items` projection

or normalized trace columns on operation events:

- folio_id
- folio_entry_id
- payment_id
- invoice_id
- billing_account_id

### Actor / source handling

Actor/source are still mostly:

- actor = system
- source = api

Later, handlers should pass explicit `OperationContext`.

### Payment / Invoice detail navigation

Payment Detail and Invoice Detail links are not implemented yet.

Audit rows expose `payment_id` and `invoice_id`, but there is no detail page navigation yet.

---

## Next Phase

Next phase:

Billing Audit Workflow Validation continued

Primary objective:

Use Folio Detail and Billing Audit as operational inspection surfaces to validate billing traceability.

Focus areas:

- reason handling
- before/after correctness
- payment traceability
- invoice traceability
- actor visibility
- operation source visibility
- detail navigation
- eventual read model / projection boundary

The objective is to answer:

- Why did this balance occur?
- Who created the entry?
- When was it created?
- Which workflow created it?
- Which payment or invoice is related?
- Why was a high-risk billing operation performed?

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