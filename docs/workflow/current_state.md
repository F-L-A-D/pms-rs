# PMS-RS Current State

## Current Repository State

Current branch baseline:

* mutable operational PMS core
* append-only behavioral/accounting history
* rebuildable projection architecture
* projection-oriented reconstruction model

---

## Projection Architecture State

Projection layer is now topology-managed and orchestration-driven.

Current projection dependency chain:

reservation
→ inventory projection
→ hotel inventory projection

Projection dependencies are now:

* explicit
* deterministic
* rebuildable
* orchestration-owned

Projection propagation semantics are centralized into topology/orchestrator layers rather than service-local propagation chains.

---

## Completed Scope (Div3)

### Operational Core

* reservation lifecycle
* stay workflow
* folio/billing workflow
* housekeeping workflow
* guest linkage
* transaction-scoped consistency

### Projection Infrastructure

* reservation search projection
* guest summary projection
* inventory projection
* hotel inventory aggregation projection

### Projection Stabilization

* projection rebuild separation
* projection refresh separation
* projection model layer introduction
* projection dependency topology
* rebuild orchestration
* refresh orchestration
* deterministic projection traversal
* projection chain rebuildability
* refresh/rebuild equivalence validation

---

## Projection Guarantees

The following guarantees are now established:

### Rebuildability

All projections are treated as disposable derived state.

Projection chains themselves are rebuildable.

### Determinism

Equivalent operational state must converge to equivalent projection state.

### Refresh/Rebuild Symmetry

Incremental refresh propagation must converge to the same state as full projection rebuild traversal.

### Authority Boundary

Projections never become operational source of truth.

Operational entities remain authoritative.

---

## Current Architectural Constraints

### Transaction Ownership

Transactions are owned by usecase/orchestration layers.

Repositories and projections consume transactions but never own them.

### Projection Ownership

Projection dependency propagation is topology-managed.

Services should not directly own downstream projection knowledge.

### Projection Semantics

Projection layers are:

* derived
* disposable
* rebuildable
* query-oriented

not operational authorities.

---

## Current Next Targets

### Projection Invalidations

Formalize which downstream projections require refresh propagation and why.

### Workflow Reconstruction

Strengthen timeline/event reconstruction guarantees.

### Projection Intelligence

Prepare projection infrastructure for:

* analytics
* forecasting
* segmentation
* CRM intelligence
