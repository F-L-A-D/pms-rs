# Current State

## Phase

Projection runtime stabilization and hospitality operational expansion have reached a stable backend foundation.

The current focus is shifting from backend-only semantic expansion to an integrated operational console phase:

- backend/frontend repository separation
- frontend console foundation
- backend health check connectivity
- Reservation Detail as the first vertical UI slice
- practical discovery of missing workflow, DTO, projection, and audit requirements through UI usage

The goal is not frontend completeness. The goal is to use a minimal console UI to expose operational gaps that are difficult to find from backend tests alone.

## Repository Structure

The repository is being organized as a SaaS-oriented workspace:

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

Backend tests are expected to remain all green before frontend work continues.

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

## Established Operational Rules

Preserve these boundaries:

- handler is the transport normalization boundary
- usecase is the transaction owner
- repository is the persistence boundary
- projection is derived/read-oriented and must not become operational authority
- append-only operational/audit history should be preferred where appropriate
- explicit enum snake_case persistence remains the convention

## Current Frontend Direction

Frontend work should begin with a minimal console, not a polished product UI.

Current frontend target:

- Vite
- React
- TypeScript
- TanStack Query
- simple API client
- backend `/health` connectivity
- Reservation Detail as the first vertical slice

The console should prioritize:

- operational visibility
- dense workflow-oriented layout
- reservation state visibility
- audit/timeline visibility
- projection visibility
- conflict/concurrency awareness
- source/actor/operation context visibility

Avoid early focus on:

- polished design
- marketing-style dashboard
- AI-generated-looking UI
- excessive cards
- animation
- mobile optimization

## First UI Slice

The first meaningful frontend slice is Reservation Detail.

It should reveal whether backend/API/projection semantics are sufficient for real operation:

- reservation summary
- stay dates
- guest/participant information
- room assignment status
- package/rate/daily stay information
- internal notes / operational notes
- audit trail
- operation change events
- conflict/concurrency status
- semantic/projection status where available

The purpose is to discover missing backend capability, not to finalize UI design.

## Current Expansion Areas

Good next areas:

- frontend `/health` check and CORS confirmation
- Reservation Detail API connection
- reservation operational visibility
- audit/timeline display around reservation workflows
- projection read APIs needed by the console
- DTO consistency for frontend consumption
- conflict/draft-preservation UX requirements
- external source/change notification semantics
- practical operational boards after Reservation Detail:
  - Arrival Board
  - In-house Board
  - Room Rack
  - Housekeeping Board
  - Billing Ledger

Areas that should remain stable:

- projection runtime and topology orchestration
- transaction ownership rules
- repository transaction behavior
- explicit enum snake_case persistence
- operational/projection authority separation
- backend as semantic authority

## Database Direction

Development currently uses SQLite.

Production is expected to use MySQL.

Frontend must not depend on database type. Database differences should remain behind backend repository/bootstrap/migration boundaries.

## Desktop Direction

Desktop support is a future Tauri wrapper over the SaaS frontend.

Desktop must not own:

- business logic
- database authority
- projection authority
- tenant authority
- sync authority

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
