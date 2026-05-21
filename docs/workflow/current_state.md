# Current State

## Phase

Projection runtime stabilization and hospitality operational expansion have reached a stable backend foundation.

The current focus has shifted from backend-only semantic expansion to an integrated operational console phase:

- backend/frontend repository separation
- frontend console foundation
- backend health check connectivity
- Reservation Search as the first operational discovery surface
- Reservation Detail as the first vertical UI slice
- practical discovery of missing workflow, DTO, projection, audit, and linked-resource requirements through UI usage

The goal is not frontend completeness. The goal is to use a minimal console UI to expose operational gaps that are difficult to find from backend tests alone.

Current reservation console validation has reached a stable checkpoint:

- Reservation Search MVP is implemented.
- Reservation Detail workflow validation is implemented.
- Reservation ↔ Folio linkage is verified.
- `linked_resources.folio_id` is visible from both Search and Detail.
- Next focus is Folio Detail Workflow Validation.

---

## Repository Structure

The repository is organized as a SaaS-oriented workspace:

```text
pms-rs/
├── backend/
│   ├── Cargo.toml
│   ├── src/
│   ├── tests/
│   └── data/
├── frontend/
│   └── console/
├── docs/
└── Cargo.toml
```

The root `Cargo.toml` is a workspace manifest:

```toml
[workspace]
members = ["backend"]
resolver = "2"
```

Backend remains the authoritative runtime and semantic layer. Frontend is an operational console that communicates through backend APIs only.

---

## Backend Status

The backend currently includes stable foundations for:

- reservation lifecycle and modification workflows
- stay/check-in/check-out workflows
- room assignment and room movement workflows
- room maintenance and out-of-order handling
- housekeeping daily state workflows
- folio, invoice, payment, receivable, and settlement boundaries
- package/rate-plan related operational support
- operational audit logs
- operation change events
- reservation edit concurrency controls
- semantic signal and activation flow
- projection rebuild/refresh parity and deterministic convergence tests
- reservation search read model
- reservation detail read model
- reservation linked resources
- automatic folio creation during reservation creation

Backend tests are expected to remain all green before frontend work continues.

---

## Established Projection Guarantees

Preserve these invariants:

- topology-managed propagation
- deterministic traversal ordering
- rebuildable projection chains
- refresh/rebuild equivalence
- transaction-scoped propagation consistency
- projection authority boundary
- projection remains derived, disposable, and non-authoritative
- operational truth remains authoritative

Projection runtime redesign is not the default task.

---

## Established Operational Rules

Preserve these boundaries:

- handler is the transport normalization boundary
- usecase is the transaction owner
- repository is the persistence boundary
- projection is derived/read-oriented and must not become operational authority
- append-only operational/audit history should be preferred where appropriate
- explicit enum snake_case persistence remains the convention

---

## Current Branch / PR

Current completed branch:

```text
feature/console-reservation-search
```

PR:

```text
#79 Add console reservation search and detail workflow validation
```

Base branch:

```text
develop
```

---

## Current Frontend Direction

Frontend work should continue with a minimal console, not a polished product UI.

Current frontend stack:

- Vite
- React
- TypeScript
- TanStack Query
- simple API client
- backend `/health` connectivity
- Reservation Search
- Reservation Detail

The console should prioritize:

- operational visibility
- dense workflow-oriented layout
- reservation state visibility
- audit/timeline visibility
- projection visibility
- linked-resource visibility
- conflict/concurrency awareness
- source/actor/operation context visibility

Avoid early focus on:

- polished design
- marketing-style dashboard
- AI-generated-looking UI
- excessive cards
- animation
- mobile optimization

---

## Reservation Search MVP

Reservation Search MVP is implemented.

Search filters currently supported:

- `external_id`
- `guest_name`
- `check_in_from`
- `check_in_to`
- `stay_date`
- `reservation_status`
- `stay_status`
- `room_class`
- `room_id`
- `booking_channel`
- `source_channel`

Search response currently includes:

- `id`
- `external_id`
- `primary_guest_id`
- `primary_guest_name`
- `check_in`
- `check_out`
- `reservation_status`
- `stay_status`
- `room_class`
- `room_id`
- `booking_channel`
- `source_channel`
- `created_at`
- `linked_resources`
  - `primary_guest_id`
  - `assigned_room_id`
  - `folio_id`

Frontend implemented:

- `ReservationListPage`
- `ReservationSearchForm`
- `ReservationSearchTable`
- `searchReservations` API client
- Search → Reservation Detail navigation

Validation completed:

- confirmed reservation
- modified reservation
- cancelled reservation
- no_show reservation
- reinstated reservation
- room assigned reservation
- room released reservation
- range search
- booking_channel search
- source_channel search

---

## Reservation Detail Validation

Reservation Detail workflow validation is implemented.

Reservation Detail currently verifies visibility for:

- reservation summary
- stay dates
- reservation status
- stay status
- room assignment
- participants
- participant details
- package breakdowns
- daily stay details
- daily revenue allocations
- notes
- traces
- audit logs
- operation events
- semantic signal visibility
- active edit sessions
- room history
- booking_channel
- source_channel
- linked_resources

`linked_resources` currently contains:

```text
primary_guest_id
assigned_room_id
folio_id
```

Verified:

- Search API returns `linked_resources.folio_id`
- Detail API returns `linked_resources.folio_id`
- Console UI displays `folio_id`
- Reservation Detail can now serve as an entry point into Billing/Folio workflow validation

---

## Folio Integration

Reservation ↔ Folio linkage is implemented and verified.

Current behavior:

```text
create_reservation
    ↓
automatic folio creation
    ↓
reservation ↔ folio linkage
```

Implemented:

- create reservation automatically creates an open folio
- active duplicate folio creation is rejected
- reservation search exposes active `folio_id`
- reservation detail exposes active `folio_id`

Validation completed:

```text
reservation creation
    ↓
folio auto creation
    ↓
duplicate folio prevention
    ↓
deposit posting
    ↓
folio entry creation
    ↓
audit logging
```

Verified:

- reservation has active folio
- search returns `folio_id`
- detail returns `folio_id`
- UI displays `folio_id`
- duplicate folio opening returns conflict
- deposit posting creates folio entry
- billing/audit logs are recorded

Important boundary:

- Folio may be created automatically.
- Charges must not be automatically posted during reservation creation.
- Balance must remain derived from folio entries.
- Reservation must not become the billing authority.

---

## Operation Types

Implemented operation types include:

```text
Create
Modify
Cancel
NoShow
Reinstate
```

Confidence profile handling is updated for the added operation types.

Validated scenarios:

- confirmed
- modified
- cancelled
- no_show
- reinstated

Room assignment lifecycle validation:

```text
confirmed  → room assigned
modified   → room assigned
cancelled  → room released
no_show    → room released
reinstated → room assignment not restored
range      → room assigned
```

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

These seeds are used to validate Search, Detail, room assignment lifecycle, audit visibility, operation event visibility, and linked-resource visibility.

---

## First UI Slice Status

The first meaningful frontend slice was Reservation Search → Reservation Detail.

This slice has now validated that backend/API/projection semantics are sufficient for basic reservation workflow inspection.

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

The purpose remains discovery of missing backend capability, not final UI design.

---

## Current Expansion Areas

Good next areas:

- Folio Detail API
- Folio Detail UI
- Reservation Detail → Folio Detail navigation
- Folio entries display
- deposit display
- room charge display
- payment display
- balance calculation display
- billing audit log visibility
- billing workflow validation
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
Folio Detail Workflow Validation
```

Primary objective:

Use Folio Detail as the next operational console slice to validate Billing workflow.

Initial Folio Detail should prioritize:

- folio id
- reservation id
- folio status
- billing_account_id
- entries
- deposits
- room charges
- payments
- balance
- audit logs
- Reservation Detail return navigation

The goal is to identify missing:

- workflow
- business logic
- navigation
- events
- DTO fields
- query models
- projections
- operational data

Do not over-polish the UI.

---

## Database Direction

Development currently uses SQLite.

Production is expected to use MySQL.

Frontend must not depend on database type. Database differences should remain behind backend repository/bootstrap/migration boundaries.

---

## Desktop Direction

Desktop support is a future Tauri wrapper over the SaaS frontend.

Desktop must not own:

- business logic
- database authority
- projection authority
- tenant authority
- sync authority

---

## Non-Goals

Do not prioritize:

- frontend completeness
- customer-facing UI
- Tauri implementation
- full auth/tenant implementation
- async propagation
- distributed invalidation
- traversal optimization
- partial rebuild optimization
- incremental projection engine
- framework extraction
- AI recommendation UI
- RMS/forecasting UI