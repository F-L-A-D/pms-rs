# PMS-RS Roadmap

---

# Div1 - Operational PMS Core

Status: DONE

Implemented:

* reservation lifecycle
* room operations
* stay operations
* folio/billing
* housekeeping
* transactional rollback consistency
* append-only billing history
* participant-aware reservation model
* UUID-based aggregate identity propagation
* transaction-scoped orchestration consistency

Validated:

* transactional rollback safety
* operational authority consistency
* append-only billing consistency
* aggregate ownership boundaries
* workflow mutation correctness

---

# Div2 - CRM / Projection Layer

Status: DONE

Implemented:

* guest timeline aggregation
* guest summary projection
* reservation search projection
* inventory projection
* hotel inventory aggregation projection
* projection materialization
* projection rebuild flow
* projection refresh flow
* projection integration tests
* projection synchronization validation
* transaction ownership policy

Validated:

* rebuild/refresh consistency
* participant-aware propagation
* behavioral aggregation consistency
* billing-driven projection refresh
* reservation-search consistency
* inventory rebuild consistency
* projection disposal safety

---

# Div3 - Workflow Validation & Projection Architecture Stabilization

Status: IN PROGRESS

## Purpose

Validate whether operational workflows remain stable under realistic integration-driven usage.

The project intentionally shifted away from frontend-first validation after workflow-level inconsistencies and query precision issues proved to be architecturally higher priority.

Div3 focuses on operational hardening, projection determinism, and workflow reconstruction integrity rather than UI implementation.

---

## Primary Focus Areas

### Workflow Consistency

Validate:

* transactional correctness
* rollback consistency
* downstream propagation consistency
* orchestration integrity
* aggregate interaction correctness
* workflow reconstruction consistency

---

### Query Hardening

Validate:

* search precision
* false-positive suppression
* operational usability
* realistic partial-match behavior
* query boundary correctness

Examples:

* guest search precision
* reservation retrieval correctness
* projection-driven query consistency

---

### Projection Architecture Stabilization

Validate:

* projection dependency topology
* deterministic traversal ordering
* rebuild orchestration
* refresh orchestration
* projection chain rebuildability
* refresh/rebuild equivalence
* downstream propagation ownership
* topology-managed dependency traversal

---

### Projection Synchronization

Validate:

* refresh consistency
* rebuild consistency
* billing propagation
* timeline propagation
* behavioral synchronization
* inventory propagation consistency
* projection recovery consistency

---

### Identity Propagation

Validate:

* UUID-based aggregate propagation
* application-owned identity generation
* repository serialization consistency
* workflow identity consistency

---

### Aggregate Boundary Validation

Validate:

* aggregate ownership correctness
* orchestration responsibilities
* behavioral propagation boundaries
* operational authority separation
* projection authority boundaries

---

## Planned Div3 Workflow Validation Targets

### Guest Workflow Validation

* create/update/search behavior
* duplicate detection edge cases
* participant propagation
* timeline consistency
* guest summary correctness

---

### Reservation Workflow Validation

* participant mutation
* modification propagation
* cancellation propagation
* room reassignment consistency
* workflow reconstruction

---

### Billing Workflow Validation

* folio lifecycle
* append-only billing history
* balance derivation
* billing projection synchronization
* rebuild consistency

---

### Projection Workflow Validation

* rebuild equivalence
* refresh equivalence
* projection disposal safety
* projection recovery consistency
* projection chain determinism
* topology traversal consistency
* downstream rebuild consistency

---

### Projection Invalidations

Implemented:

* invalidation semantics
* selective downstream propagation
* propagation ownership guarantees
* dependency invalidation policy
* authoritative rebuild-equivalence boundaries
* scoped rebuild convergence semantics
* refresh/rebuild scoped symmetry
* boundary-scoped correctness guarantees
* execution consumption of semantic rebuild boundaries

---

## Explicitly Out of Scope

The following are intentionally excluded from Div3:

* frontend completeness
* SPA architecture
* UI polish
* authentication
* production frontend design
* dashboard optimization
* asynchronous distributed consistency

UI layers are considered temporary validation tooling only.

---

# Div4 - Segmentation Foundation

Planned:

* behavioral segmentation
* guest clustering
* CRM interpretation models
* projection-oriented behavioral grouping
* behavioral reconstruction models
* projection-driven customer intelligence

---

# Div5 - Forecasting & Intelligence

Planned:

* forecasting models
* behavioral prediction
* operational intelligence
* revenue intelligence
* operational decision support
* occupancy analytics
* pickup analytics
* lead-time analytics
* projection-driven forecasting infrastructure

---

# Long-Term Architecture Direction

Target architecture:

* mutable operational PMS core
* append-only behavioral/accounting history
* deterministic rebuildable projection system
* projection-oriented reconstruction architecture
* workflow-driven validation infrastructure
* analytics-ready projection dependency topology

Operational entities remain authoritative.

Projection systems remain:

* derived
* disposable
* rebuildable
* topology-managed
* orchestration-driven