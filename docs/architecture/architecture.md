# PMS-RS Operational Truth vs Behavioral Projection

The architecture explicitly separates:

- operational truth
- behavioral history
- derived intelligence

## Operational Truth

Mutable operational entities representing current authoritative state.

Examples:

- reservations
- reservation_guest_relations
- rooms
- folios

Operational truth is authoritative for:

- reservation ownership
- occupancy state
- room assignment
- billing responsibility
- operational workflows

Operational truth may be:

- modified
- reassigned
- corrected
- cancelled

Operational truth must remain mutable where operational reality requires correction.

---

## Behavioral History

Append-only behavioral/accounting records.

Examples:

- guest timeline
- folio entries

Behavioral history preserves reproducibility of:

- guest behavior
- operational actions
- accounting transitions

Behavioral history is projection-oriented and non-authoritative.

Behavioral history must not replace operational truth.

---

## Behavioral Projections

Derived read models generated from:

- operational truth
- behavioral history

Examples:

- guest metrics
- CRM projections
- segmentation
- forecasting features
- behavioral analytics

Behavioral projections are:

- rebuildable
- disposable
- versionable
- non-authoritative

Projection inconsistency must never corrupt operational truth.

---

# Projection Rebuildability

CRM and behavioral projections must be rebuildable from:

- operational tables
- append-only behavioral history

Projection persistence exists for:

- query efficiency
- analytics
- segmentation
- forecasting
- personalization

Projection storage must not introduce independent business authority.

Projection rebuildability is required for:

- schema evolution
- metric refinement
- behavioral reinterpretation
- segmentation redesign
- forecasting improvements

---

# Aggregate Reconstruction

Operational aggregates may be reconstructed from:

- operational tables
- relation tables

Examples:

- reservation_guest_relations
- room assignment relations
- future corporate relations

Aggregate reconstruction must preserve authoritative ownership boundaries.

Behavioral projections must not become reconstruction authority.

Projection-derived state must never replace operational truth during aggregate reconstruction.

---

# Behavioral Projection Constraints

Behavioral projections exist to support:

- CRM
- analytics
- forecasting
- segmentation
- personalization
- operational intelligence

Behavioral projections must not:

- own operational workflows
- mutate operational authority
- become transactional truth
- replace mutable operational state

Projection inconsistency must not block:

- operational correction
- reservation reassignment
- room reassignment
- guest merge
- billing correction

Operational correctness has priority over projection consistency.

---

# CRM Projection Direction

CRM projections exist to support:

- behavioral understanding
- personalization
- segmentation
- forecasting
- revenue intelligence
- operational decision support

CRM projections derive from:

- operational truth
- behavioral history

CRM projections are read-oriented behavioral models.

CRM projections must not directly own:

- reservation authority
- billing authority
- room authority
- operational workflow state

CRM projections are intelligence layers rather than operational transaction layers.

---

## Transaction Ownership Policy

Usecases and orchestration services own transaction boundaries.

Repositories, projections, materializers, and rebuild flows consume existing transactions only.

This policy exists to guarantee:

* single workflow consistency
* projection synchronization
* replay compatibility
* future event sourcing compatibility

---

## Projection Philosophy

Projections are rebuildable query models.

Projections:

* may be deleted and rebuilt
* are not the source of truth
* exist for operational query optimization
* exist for workflow support

Operational entities and append-only history remain authoritative.

---

## Projection Dependency Principles

Projection dependencies are architecture primitives.

Projection dependency graphs must remain:

* explicit
* deterministic
* rebuildable
* topology-managed
* orchestration-owned

Projection dependency traversal must never rely on implicit service-to-service propagation.

---

## Projection Topology

Projection chains are modeled explicitly through topology definitions.

Example:

reservation
→ inventory projection
→ hotel inventory projection

Traversal ordering is centrally managed through topology/orchestrator layers.

---

## Projection Refresh Semantics

Refresh propagation represents incremental downstream synchronization.

Refresh traversal is downstream-only.

Example:

InventoryProjection refresh
→ HotelInventoryProjection refresh

The originating projection is assumed already updated.

---

## Projection Rebuild Semantics

Rebuild traversal represents full reconstruction from operational authority.

Rebuild traversal includes:

* self rebuild
* downstream rebuild

Example:

InventoryProjection rebuild
→ HotelInventoryProjection rebuild

Rebuilds always originate from operational source-of-truth state.

---

## Refresh/Rebuild Symmetry

Incremental propagation and full rebuild traversal must converge to identical projection state.

This is treated as an architectural invariant.

Equivalent operational state must always produce equivalent projection chain state.

---

## Projection Authority Boundary

Projections are not operational authorities.

Downstream projections must never become canonical operational state.

Operational entities remain authoritative.

Projection layers are treated as:

* derived
* disposable
* rebuildable
* query-oriented

---

## Projection Orchestration Ownership

Projection dependency propagation is owned by topology/orchestrator layers.

Projection services should only manage:

* local projection refresh
* local projection materialization
* local projection persistence

Services must not directly own downstream dependency propagation semantics.

---

## Projection Consistency Guarantees

The architecture guarantees:

* deterministic projection reconstruction
* rebuildable projection chains
* topology-consistent traversal ordering
* refresh/rebuild equivalence
* transaction-scoped propagation consistency

---

## Architectural Constraints

The following are intentionally prohibited unless explicitly redesigned:

* repositories owning transactions
* projections committing transactions
* projections becoming the source of truth
* business logic inside API handlers
* direct projection mutation without rebuildability
* bypassing timeline/event recording for behavioral changes