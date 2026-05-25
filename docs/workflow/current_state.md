# Current State

## Phase

Projection runtime stabilization and hospitality operational expansion have reached a stable backend foundation.

The current focus is the integrated operational console phase:

- backend/frontend repository separation
- frontend console foundation
- backend health check connectivity
- Reservation Search as the first operational discovery surface
- Reservation Detail as the first vertical UI slice
- Folio Detail as the first Billing workflow slice
- Billing Audit as the first billing traceability validation surface
- practical discovery of missing workflow, DTO, projection, audit, linked-resource, and operational lifecycle requirements through UI usage

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
- Billing lifecycle foundation has reached a stable backend checkpoint.
- Payment lifecycle validation is implemented.
- Deposit lifecycle validation is implemented.
- Invoice / Receivable lifecycle validation is implemented.
- Billing audit reason and before/after correctness have been improved for the current scope.

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

## Billing Lifecycle Validation

Billing lifecycle validation has reached a stable backend checkpoint.

The current backend supports the core billing operational lifecycle needed for console inspection:

Invoice / Receivable:

- create_invoice
- void_invoice
- dispute_receivable
- resolve_receivable_dispute
- write_off_receivable
- receivable detail
- receivable aging

Payment lifecycle:

- receive_payment
- allocate_existing_payment
- reverse_payment_allocation
- refund_payment

Deposit lifecycle:

- receive_deposit
- apply_deposit_to_receivable
- reverse_deposit_application
- refund_deposit

Current lifecycle model:

Payment:

```text
receive_payment
→ allocate_existing_payment
→ reverse_payment_allocation
→ refund_payment
````

Deposit:

```text
receive_deposit
→ apply_deposit_to_receivable
→ reverse_deposit_application
→ refund_deposit
```

Implemented backend capabilities:

* Payment lifecycle foundation.
* Payment entity supports:

  * unapplied_amount
  * refunded_amount
  * status
* PaymentStatus supports:

  * unapplied
  * partially_applied
  * applied
  * partially_refunded
  * refunded
  * voided
* `create_payment` now means receiving an unapplied payment.
* `allocate_existing_payment` applies an existing payment to a receivable.
* `reverse_payment_allocation` reverses payment allocation.
* `refund_payment` refunds unapplied payment amount.
* PaymentRefund entity and repository are implemented.
* Payment refund table is implemented.

Implemented Deposit capabilities:

* Deposit lifecycle foundation.
* Deposit entity supports:

  * unapplied_amount
  * refunded_amount
  * status
* DepositStatus supports:

  * held
  * partially_applied
  * applied
  * partially_refunded
  * refunded
  * forfeited
  * voided
* `create_deposit` means receiving a held deposit.
* `apply_deposit_to_receivable` applies deposit amount to a receivable.
* `reverse_deposit_application` reverses a deposit application.
* `refund_deposit` refunds unapplied deposit amount.
* DepositApplication entity and repository are implemented.
* DepositRefund entity and repository are implemented.
* Deposit application table is implemented.
* Deposit refund table is implemented.

Implemented audit actions:

Payment:

* `payment.receive`
* `payment.allocate`
* `payment.allocation.reverse`
* `payment.refund`

Deposit:

* `deposit.receive`
* `deposit.apply`
* `deposit.application.reverse`
* `deposit.refund`

Invoice / Receivable:

* `invoice.issue`
* `invoice.void`
* `receivable.dispute`
* `receivable.dispute.resolve`
* `receivable.write_off`

Implemented operation types:

* ReceivePayment
* AllocateExistingPayment
* ReversePaymentAllocation
* RefundPayment
* ReceiveDeposit
* ApplyDepositToReceivable
* ReverseDepositApplication
* RefundDeposit
* IssueInvoice
* VoidInvoice
* DisputeReceivable
* ResolveReceivableDispute
* WriteOffReceivable
* AssignBillingAccount
* CloseFolio

Validated payment lifecycle:

receive_payment:

* operation_type: receive_payment
* action: payment.receive
* status: unapplied
* unapplied_amount set to payment amount

allocate_existing_payment:

* operation_type: allocate_existing_payment
* action: payment.allocate
* payment_unapplied_amount changes to 0
* payment_status changes to applied
* receivable_outstanding_amount changes to 0
* receivable_status changes to settled

reverse_payment_allocation:

* operation_type: reverse_payment_allocation
* action: payment.allocation.reverse
* allocation reversed_at changes from null to timestamp
* payment_unapplied_amount is restored
* payment_status is restored
* receivable_outstanding_amount is restored
* receivable_status is restored

refund_payment:

* operation_type: refund_payment
* action: payment.refund
* payment_refunded_amount changes from 0
* payment_unapplied_amount decreases
* payment_status changes to partially_refunded or refunded

Validated deposit lifecycle:

receive_deposit:

* operation_type: receive_deposit
* action: deposit.receive
* status: held
* unapplied_amount set to deposit amount

apply_deposit_to_receivable:

* operation_type: apply_deposit_to_receivable
* action: deposit.apply
* deposit_unapplied_amount decreases
* deposit_status changes to partially_applied or applied
* receivable_outstanding_amount decreases
* receivable_status updates accordingly

reverse_deposit_application:

* operation_type: reverse_deposit_application
* action: deposit.application.reverse
* deposit_application_reversed_at changes from null to timestamp
* deposit_unapplied_amount is restored
* deposit_status is restored
* receivable_outstanding_amount is restored
* receivable_status is restored

refund_deposit:

* operation_type: refund_deposit
* action: deposit.refund
* deposit_refunded_amount changes from 0
* deposit_unapplied_amount decreases
* deposit_status changes to partially_refunded or refunded

Confirmed refund_deposit validation:

* deposit_refunded_amount: 0 → 1500
* deposit_unapplied_amount: 5000 → 3500
* deposit_status: held → partially_refunded

---

## Billing Audit Workflow Validation

Billing Audit Workflow Validation has reached a stable console checkpoint.

The console supports the operational path:

Reservation Search
→ Reservation Detail
→ Folio Detail
→ Billing Audit

Billing Audit currently validates traceability for the main billing workflow:

payment / deposit received
→ payment / deposit allocated or applied
→ allocation or application reversed
→ payment / deposit refunded
→ billing account assigned
→ folio closed
→ invoice issued
→ invoice voided
→ receivable disputed / resolved / written off

Implemented backend capabilities:

* Billing Audit API from Folio Detail.
* `BillingAuditResponse` with business-readable display fields:

  * amount
  * payment_method
  * payment_reference
  * invoice_number
  * issued_amount
  * billing_account_id
  * billing_account_name
* operation event recording for:

  * create_payment
  * create_deposit
  * allocate_existing_payment
  * reverse_payment_allocation
  * refund_payment
  * apply_deposit_to_receivable
  * reverse_deposit_application
  * refund_deposit
  * create_invoice
  * void_invoice
  * dispute_receivable
  * resolve_receivable_dispute
  * write_off_receivable
  * assign_billing_account
  * close_folio
* billing account creation API.
* close folio handler and route.
* assign billing account request cleanup:

  * `folio_id` is sourced from Path
  * request body carries only `billing_account_id`
* fixed operation type persistence typo:

  * `assigun_billing_account` → `assign_billing_account`

Implemented frontend capabilities:

* Billing Audit section on Folio Detail.
* Billing Audit table with business-readable columns:

  * Amount
  * Method
  * Reference
  * Invoice
  * Billing Account
  * Actor
  * Source
  * Reason
  * Trace
* Reservation Detail folio grouping:

  * Active Folios
  * Other Folios
* Closed folios can be reached from Reservation Detail.

Validated Billing Audit response for billing lifecycle scenarios:

* receive_payment
* receive_deposit
* allocate_existing_payment
* reverse_payment_allocation
* refund_payment
* apply_deposit_to_receivable
* reverse_deposit_application
* refund_deposit
* assign_billing_account
* close_folio
* issue_invoice
* void_invoice
* dispute_receivable
* resolve_receivable_dispute
* write_off_receivable

Verified display fields:

receive_payment / receive_deposit:

* amount
* payment_method
* payment_reference

payment allocation / refund:

* payment_id
* amount
* before_json
* after_json
* changed_fields_json
* reason

deposit application / refund:

* aggregate_id
* amount
* before_json
* after_json
* changed_fields_json
* reason

assign_billing_account:

* billing_account_id
* billing_account_name

close_folio:

* billing_account_id
* billing_account_name
* status transition

issue_invoice:

* invoice_number
* issued_amount
* billing_account_name
* receivable_id visibility from Invoice Detail

void_invoice:

* invoice_number
* issued_amount
* billing_account_name
* invoice status transition
* receivable status transition

Current Billing Audit response includes:

* operation_id
* folio_id
* folio_entry_id
* payment_id
* invoice_id
* aggregate_type
* aggregate_id
* operation_type
* actor
* actor_id
* source
* action
* reason
* amount
* payment_method
* payment_reference
* invoice_number
* issued_amount
* billing_account_id
* billing_account_name
* before_json
* after_json
* changed_fields_json
* occurred_at

Folio Audit resolution supports:

* folio_entry → folio_id
* payment → folio_id
* payment_allocation → payment → folio_id
* payment_refund → payment → folio_id
* deposit → folio_id
* deposit_application → deposit → folio_id
* deposit_refund → deposit → folio_id
* invoice → folio_id
* receivable → invoice → folio_id

---

## Reservation Detail Folio Links

Reservation Detail now distinguishes between the backward-compatible active folio link and all linked folios.

Existing field:

* `linked_resources.folio_id`

Meaning:

* Open / Locked representative folio.
* Used for backward compatibility and active folio visibility.

New field:

* `folios`

Meaning:

* All folios linked to the reservation, including Closed folios.

Console display:

* Active Folios
* Other Folios

This resolves the issue where closed folios disappeared from Reservation Detail after invoice workflow validation.

---

## Seed / Console Validation Data

Console validation seed scenarios are available for:

* confirmed
* modified
* cancelled
* no_show
* reinstated
* range search
* room assigned
* room unassigned
* note
* trace
* room charge
* tax charge
* deposit received
* payment received
* manual adjustment
* balance validation
* billing account creation
* billing account assignment
* folio close
* invoice issue
* invoice void
* receivable dispute
* receivable dispute resolve
* receivable write-off
* payment allocation
* payment allocation reverse
* payment refund
* deposit application
* deposit application reverse
* deposit refund
* billing audit traceability

These seeds are used to validate Search, Detail, room assignment lifecycle, audit visibility, operation event visibility, linked-resource visibility, billing visibility, invoice visibility, receivable visibility, payment/deposit lifecycle visibility, and balance calculation.

Current Billing validation seed expectations:

confirmed reservation:

* charges 13200
* payments 5000
* balance 8200
* audit: receive_deposit

range reservation:

* charges 20000
* payments 20000
* balance 0
* audit:

  * apply_payment
  * assign_billing_account
  * close_folio
  * issue_invoice
  * void_invoice

past reservation:

* charges 8000
* payments 8000
* balance 0
* audit: apply_payment

payment/deposit lifecycle validation:

* payment.receive
* payment.allocate
* payment.refund
* deposit.receive
* deposit.apply
* deposit.application.reverse
* deposit.refund

---

## First UI Slice Status

The first meaningful frontend slice was Reservation Search → Reservation Detail.

This slice validated that backend/API/projection semantics are sufficient for reservation workflow inspection.

The second meaningful frontend slice is Reservation Detail → Folio Detail.

This slice validated that billing workflows are inspectable through the console and that balance derivation behaves correctly from operational folio entries.

The third meaningful frontend slice is Folio Detail → Billing Audit.

This slice validated that billing workflows can be traced through operation events, audit logs, and business-readable display fields.

Confirmed visible from the UI:

* reservation summary
* booking_channel
* source_channel
* room assignment status
* participants
* participant details
* linked resources
* active folio link
* all folio links
* closed folio links
* notes
* traces
* audit logs
* operation events
* folio entries
* room charges
* tax charges
* deposits
* payments
* balance
* billing audit events
* billing account assignment
* folio close
* invoice issue
* invoice void
* payment lifecycle events
* deposit lifecycle events
* receivable lifecycle events
* payment method
* payment reference
* invoice number
* billing account name
* operation reason
* before/after changes

The purpose remains discovery of missing backend capability, not final UI design.

---

## Current Stable Areas

Stable backend foundations:

* projection runtime and topology orchestration
* transaction ownership rules
* repository transaction behavior
* explicit enum snake_case persistence
* operational/projection authority separation
* backend as semantic authority
* linked-resource separation between active representative link and all linked resources
* Reservation Search MVP
* Reservation Detail workflow validation
* Folio Detail workflow validation
* Billing Audit workflow validation
* Billing lifecycle command foundation
* Payment lifecycle foundation
* Deposit lifecycle foundation
* Invoice / Receivable lifecycle foundation

Stable Billing lifecycle capabilities:

* Payment receive / allocate / reverse allocation / refund
* Deposit receive / apply / reverse application / refund
* Invoice issue / void
* Receivable dispute / resolve / write-off
* Folio close
* Billing account assignment
* Folio Audit traceability

---

Stable Room / Housekeeping capabilities:

* Room List / Room Detail console validation
* RoomDailyState visibility by service_date
* Room assignment visibility derived from Reservation.room_id
* Assigned / occupied separation
* No-Show linked room warning visibility
* Check-in / check-out console actions
* Inspected-room requirement before check-in
* Check-in → occupied
* Check-out → vacant + dirty
* Housekeeping lifecycle actions from Room Detail
* Room maintenance actions from Room Detail
* Room Move from Reservation Detail
* Room Move conflict visibility
* Room Move transition / audit / operation event recording

---

## Current Known Gaps

### Billing Audit query model

Billing Audit API currently uses operation event scan plus `after_json` extraction.

This is acceptable for validation.

Later candidates:

* `billing_audit_items` projection

or normalized trace columns on operation events:

* folio_id
* folio_entry_id
* payment_id
* invoice_id
* billing_account_id

### Actor / source handling

Actor/source are still mostly:

* actor = system
* source = api

Later, handlers should pass explicit `OperationContext`.

### Payment / Invoice detail navigation

Payment Detail and Invoice Detail links are not implemented yet.

Audit rows expose `payment_id` and `invoice_id`, but there is no detail page navigation yet.

### Billing UI operation forms

Billing lifecycle backend commands exist, but not all of them have dedicated console operation forms.

Examples:

* payment refund
* payment allocation reverse
* deposit application
* deposit application reverse
* deposit refund
* receivable dispute / resolve / write-off

These are currently validated mainly through API seed and audit inspection.

### Deposit forfeit / void

Deposit status supports forfeited and voided, but forfeit / void workflows are not yet implemented as operational commands.

---

### Business Date / Night Audit foundation

The system does not yet have a current business date authority.

Current `service_date` usage is mostly a query / room daily state filter, not the authoritative system business date.

Because of this, the following are still possible and must be addressed next:

* future reservation check-in
* future reservation check-out
* future housekeeping transitions
* future maintenance transitions
* future room move effective dates
* operation execution without business date validation

The next backend foundation should introduce a business date / service date authority and night audit lifecycle:

* current open business date
* business date close
* next business date open
* operation validation against current business date
* minimum night audit workflow
* billing day-boundary foundation

This should be handled before expanding additional date-sensitive workflows such as turnover, due-out operations, housekeeping task assignment, or advanced room availability.

---

## Room / Housekeeping Workflow Validation

Room / Housekeeping Console Validation Foundation has reached a stable checkpoint.

This phase used the operational console to validate room visibility, room assignment visibility, housekeeping state transitions, stay actions, room move behavior, and missing workflow boundaries.

The purpose was not final UI design. The purpose was to expose missing operational workflows and clarify authority boundaries through console usage.

Implemented backend capabilities:

* `GET /rooms?service_date=YYYY-MM-DD` includes RoomDailyState visibility.
* `GET /rooms/:id?service_date=YYYY-MM-DD` includes RoomDailyState visibility.
* Room assignment visibility is derived from `Reservation.room_id`.
* RoomDailyState does not store assignment status.
* Assigned and occupied are explicitly separated.
* Assigned but not occupied is representable.
* No-Show is not treated as an active assignment.
* No-Show with linked `reservation.room_id` is shown as a soft warning:
  * `no_show_reservation_still_linked_to_room`
* Cancel route was changed from:
  * `DELETE /reservations/:id`
  to:
  * `POST /reservations/:id/cancel`
* Check-in now requires the assigned room to be inspected.
* Dirty / Cleaning / Cleaned rooms reject check-in.
* Inspected rooms allow check-in.
* Check-in updates RoomDailyState occupancy to occupied.
* Check-out updates RoomDailyState occupancy to vacant and housekeeping to dirty.
* Room Move workflow is implemented and validated.
* Room Move updates:
  * old room: vacant
  * old room on effective_date: dirty
  * new room: occupied
  * `Reservation.room_id`: updated to new room
* Room Move records:
  * `ReservationTransition::RoomMoved`
  * `TimelineEventType::RoomMoved`
  * audit log action: `stay.room_move`
  * `OperationChangeEvent` with `OperationType::RoomMoved`
* `OperationType::RoomMoved` is implemented.

Implemented frontend console capabilities:

* Room List Page.
* `/rooms` route.
* `service_date` filter.
* Room List calls:
  * `GET /rooms?service_date=...`
* Room List displays:
  * room_no
  * room_class
  * assignment_status
  * linked reservation
  * stay_status
  * warning
  * occupancy_status
  * housekeeping_status
* Room Detail Page.
* `/rooms/:roomId?service_date=...` route.
* Room Detail displays:
  * room identity
  * assignment visibility
  * daily_state
* Room Detail actions:
  * Check In
  * Check Out
  * Mark Dirty
  * Start Cleaning
  * Finish Cleaning
  * Inspect
  * Mark Out Of Order
  * Return To Service
* Reservation Detail actions:
  * Check In
  * Check Out
  * Room Move
* Room Move form currently uses target room ID directly.
  * Room number selector is intentionally deferred as a UI improvement.

Validated backend tests:

* RoomDailyState visibility tests.
* Room assignment visibility tests.
* No-Show linked room warning visibility.
* Check-in requires inspected room.
* Dirty / Cleaning / Cleaned rooms reject check-in.
* Inspected rooms allow check-in.
* Check-in sets occupied.
* Check-out sets vacant + dirty.
* Room Move updates source / target room daily state.
* Room Move rejects occupied target room.
* Room Move records transition.
* Room Move records audit log.
* Room Move records operation event.
* `cargo test` passed.

Validated frontend checks:

* Room List manual check OK.
* Room Detail manual check OK.
* Reservation Detail stay actions manual check OK.
* Room Move manual check OK.
* Frontend console build passed.

Important design decisions:

* Room is a static master.
* RoomDailyState is the daily authority for occupancy / housekeeping.
* Assignment truth is `Reservation.room_id`.
* Assignment visibility is derived in read responses.
* Projection/read model is not truth.
* Assigned is not the same as occupied.
* No-Show is not an active assignment.
* No-Show may still retain `reservation.room_id` for historical linkage and warning visibility.
* Check-in is only allowed when the room is inspected.
* Room Move is an operational workflow on checked-in stays.
* Room Move currently requires room ID input in the console; room number based selection is deferred.

Known limitations intentionally left for later:

* Semantic Signal materialization is out of scope for this phase.
* Current `service_date` filter is a UI/query date, not yet the system business date.
* Future-dated operations are still possible because the system does not yet have a current business date authority.
* Night audit / business date close-open lifecycle is not implemented.
* Room number selector for room move is not implemented.
* Housekeeping task assignment model is not implemented.

## Next Phase

Next phase:

Business Date / Night Audit Foundation

Primary objective:

Introduce an authoritative system business date so operational workflows cannot be executed against arbitrary future service dates.

The immediate problem is that the console and backend currently allow future-dated operational actions, such as:

* check-in for a future reservation
* check-out outside the current business date
* housekeeping state changes for future service dates
* maintenance state changes for future service dates
* room move effective dates unrelated to the current business date

The next phase should establish the system-level date authority before expanding additional Room / Housekeeping workflows.

Core concept:

```text
BusinessDate / OperationalServiceDate
= the current system business date used by operational workflows
```

This must be distinct from:


```text
Reservation.check_in / check_out
RoomDailyState.service_date
Room List service_date filter
Audit affected_service_dates
```

Initial backend scope:

* Add a business date entity/table.
* Store the current open business date.
* Provide API to get the current business date.
* Provide minimal night audit close/open command.
* Enforce current business date validation for:
  * check-in
  * check-out
  * room move
  * housekeeping actions
  * maintenance actions
* Add operational audit for night audit close/open.
* Add integration tests.

Initial frontend scope:

- Display current business date in the console.
- Default Room List / Room Detail service_date to current business date.
- Keep future service_date viewing possible if needed.
- Prevent or surface backend rejection for future-dated operational actions.
- Optionally expose a minimal Night Audit action after backend validation is stable.

Important design direction:

BusinessDate is the authority for "what operational date the system is currently on".
RoomDailyState remains the authority for daily room occupancy / housekeeping.
Reservation remains the authority for stay dates and assignment.
Projection / Semantic Signal materialization remains out of scope for this phase.
The goal is operational correctness and workflow boundary discovery, not final UI polish.

Do not proceed into turnover, housekeeping task assignment, room number selector polish, or advanced availability logic until the business date / night audit foundation is in place.