# PMS-RS Testing

## Principles

* integration-first testing
* transactional consistency validation
* API-level behavioral verification
* operational flow validation

---

## Current Coverage

### Reservation

* reservation create
* reservation modify
* reservation cancel
* invalid guest validation

### Stay

* room assignment
* check-in

### Billing

* open folio
* room charge posting
* payment posting
* balance calculation

### Housekeeping

* dirty transition
* cleaning lifecycle

### Guest

* guest creation
* guest retrieval
* guest update
* guest search

### Behavioral Projection

* timeline propagation
* behavioral event consistency
* guest metrics derivation
* empty projection handling

---

## Testing Strategy

Behavioral projections are validated through API-level integration tests rather than isolated repository tests.

Operational consistency is validated through transactional integration flows.

### Div1

Focus:

* operational correctness
* transactional consistency
* mutation integrity

### Div2

Focus:

* projection consistency
* rebuild correctness
* timeline integrity

### Div3

Focus:

* workflow stability
* UI-driven operational validation
* search responsiveness
* query model sufficiency

---

## Projection Testing Policy

Projection tests must verify:

* materialization correctness
* refresh correctness
* rebuild correctness

Projection rebuild tests must guarantee that:

* projections may be deleted
* rebuild restores equivalent query state

---

## Integration Test Philosophy

Integration tests are preferred over isolated mocking.

The project prioritizes:

* real workflow consistency
* transaction behavior
* projection synchronization

over isolated unit-level mocking.

---

### Scoped Rebuild Invariants

Projection invalidation tests must verify:

* authoritative rebuild-equivalence boundaries
* scoped convergence semantics
* refresh/rebuild symmetry
* topology-owned boundary derivation
* boundary-scoped correctness guarantees
* execution consumption of semantic rebuild boundaries

Scoped rebuild tests must guarantee that:

* affected projection boundaries remain deterministic
* refresh propagation fulfills rebuild-equivalence contracts
* topology remains authoritative for boundary expansion
* projection state outside affected boundaries remains intentionally outside scoped convergence guarantees