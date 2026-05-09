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

---

# Div2 - CRM / Projection Layer

Status: DONE

Implemented:

* guest timeline aggregation
* guest summary projection
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

---

# Div3 - Workflow Validation & Query Hardening

Status: IN PROGRESS

## Purpose

Validate whether operational workflows remain stable under realistic integration-driven usage.

The project intentionally shifted away from frontend-first validation after workflow-level inconsistencies and query precision issues proved to be architecturally higher priority.

Div3 focuses on operational hardening rather than UI implementation.

---

## Primary Focus Areas

### Workflow Consistency

Validate:

* transactional correctness
* rollback consistency
* downstream propagation consistency
* orchestration integrity
* aggregate interaction correctness

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

### Projection Synchronization

Validate:

* refresh consistency
* rebuild consistency
* billing propagation
* timeline propagation
* behavioral synchronization

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

---

## Explicitly Out of Scope

The following are intentionally excluded from Div3:

* frontend completeness
* SPA architecture
* UI polish
* authentication
* production frontend design
* dashboard optimization

UI layers are considered temporary validation tooling only.

---

# Future Divisions

## Div4 - Segmentation Foundation

Planned:

* behavioral segmentation
* guest clustering
* CRM interpretation models
* projection-oriented behavioral grouping

---

## Div5 - Forecasting & Intelligence

Planned:

* forecasting models
* behavioral prediction
* operational intelligence
* revenue intelligence
* operational decision support