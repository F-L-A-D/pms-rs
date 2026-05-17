# Architecture Principles

## Authority Model

Operational truth is mutable authoritative state. Examples include reservations, rooms, stays, folios, billing accounts, invoices, receivables, and room daily states.

Behavioral and accounting history preserves reconstruction and auditability. It supports analysis, but it does not replace operational truth.

Projections support query efficiency, CRM, segmentation, forecasting, and analytics. They are derived, rebuildable, disposable, versionable, and non-authoritative.

Operational correctness has priority over projection consistency.

## Transaction Ownership

Transactions are owned only by:

- usecases
- orchestration layers

Repositories, materializers, projection components, and semantic objects consume existing transactions. They must not begin, commit, or roll back transactions.

## Projection Philosophy

Projection propagation is explicit, deterministic, topology-managed, and orchestrator-owned.

Projection services may perform local materialization, local refresh, local persistence, and local rebuild logic. They must not call downstream projection services directly.

Refresh means incremental downstream synchronization. Rebuild means reconstruction from authoritative operational state. Refresh and rebuild must converge to equivalent projection state for equivalent operational truth.

## Domain Boundaries

Do not collapse operational concepts that carry different authority.

Billing and settlement remain separated:

- Folio: operational billing container
- BillingAccount: settlement responsibility owner
- Invoice: immutable settlement snapshot
- Receivable: outstanding settlement liability
- Payment: operational settlement mutation

Room and room state remain separated:

- Room: static room identity and structural facts
- RoomDailyState: date-scoped room occupancy and housekeeping state

## Forbidden Patterns

Do not introduce:

- projection authority over operational workflows
- hidden transaction ownership
- implicit downstream projection propagation
- projection-derived settlement authority
- timeline/history records as current operational truth
- generic framework abstractions
- service locator patterns
- speculative event buses
- generic workflow engines

Avoid files or abstractions named `common.rs`, `utils.rs`, `helper.rs`, `BaseService`, `GenericRepository`, or manager-style wrappers unless a concrete local domain concept requires them.

## Current Non-Goals

Do not optimize or redesign the runtime unless explicitly requested.

Not current priorities:

- frontend completeness
- async propagation
- distributed consistency
- traversal optimization
- framework extraction
- generic infrastructure abstraction
- partial rebuild optimization
- cache-aware propagation pruning
