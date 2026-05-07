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