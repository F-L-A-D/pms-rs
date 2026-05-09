# PMS-RS Current State

## Completed

### Div1 - Operational PMS Core

Implemented:

* reservation lifecycle
* reservation modification
* reservation cancellation
* participant-authoritative reservation model
* room assignment
* room occupancy management
* check-in / check-out
* housekeeping lifecycle
* folio management
* append-only folio entries
* room charge posting
* payment posting
* balance derivation
* transactional rollback consistency
* guest timeline event recording

---

### Div2 - CRM / Projection Layer

Implemented:

* guest timeline aggregation
* guest summary projection
* projection materialization
* projection rebuild flow
* projection refresh flow
* projection integration tests
* participant-aware behavioral propagation
* transaction ownership unification

---

## Recently Validated

The latest workflow hardening phase validated:

* UUID-based aggregate identity propagation
* application-owned identity generation
* repository-level UUID serialization consistency
* billing projection synchronization
* rebuild/refresh equivalence
* realistic workflow-driven integration testing
* precise guest search behavior
* false-positive suppression in search queries

The architecture now consistently uses:

* UUID-based internal aggregate identities
* usecase-owned aggregate creation
* repository-owned SQLite string mapping

Workflow propagation is now validated through actual downstream aggregate usage rather than fixed test-generated identities.

---

## Transaction Boundary Policy

Transaction boundaries are owned by:

* usecases
* orchestration services

Repositories, projection services, materializers, and rebuild flows consume existing transactions only.

This guarantees:

* workflow consistency
* projection synchronization
* replay compatibility
* rebuild compatibility
* future event sourcing compatibility

Projection layers must never own operational authority.

---

## Current Architecture Direction

The system currently consists of:

* mutable operational PMS core
* append-only behavioral/accounting history
* rebuildable projection layer
* projection-oriented CRM foundation
* workflow-oriented orchestration layer

---

## Current Validation Focus

The project focus has shifted from lightweight UI experimentation toward workflow validation and operational hardening.

Primary validation areas:

* transactional workflow consistency
* aggregate boundary validation
* projection synchronization consistency
* realistic operational query behavior
* workflow reconstruction consistency
* downstream identity propagation
* operational search precision
* projection rebuild equivalence

---

## Div3 - Workflow Validation & Query Hardening

Status: IN PROGRESS

### Completed

Validated:

* reservation_search projection consistency
* projection refresh/rebuild equivalence
* inventory projection rebuild flow
* room_class × date inventory semantics
* hotel-wide inventory aggregation projection
* downstream projection chaining
* projection-owned inventory consistency
* realistic workflow-driven projection synchronization
* projection model separation from operational domain models

The system now supports:

reservation
→ inventory projection
→ hotel-wide inventory projection

as a rebuildable projection chain.

Projection layers are now explicitly treated as:

* derived state
* rebuildable cache
* non-authoritative operational views

rather than operational source-of-truth models.

---

### Remaining Scope

Div3 remaining focus areas:

* operational query optimization
* projection rebuild orchestration hardening
* inventory availability policy separation
* hotel-wide occupancy analytics
* projection replay scalability
* event-oriented projection migration preparation
* query pagination/sorting validation
* multi-projection consistency validation
* search ranking/noise suppression refinement

---

### Explicitly Deferred

Out of current Div3 scope:

* production-grade frontend implementation
* authorization/authentication
* external OTA/channel integration
* RMS implementation
* pricing/revenue optimization
* distributed/event-stream infrastructure
* production observability stack

UI remains validation-oriented tooling only.
