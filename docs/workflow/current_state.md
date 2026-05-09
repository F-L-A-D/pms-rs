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

Div3 is no longer considered a UI implementation phase.

The previous lightweight validation UI direction was intentionally reduced in scope after identifying that the highest architectural value comes from workflow-level validation rather than frontend completeness.

Div3 now focuses on:

* realistic workflow pressure testing
* search/query hardening
* aggregate boundary validation
* projection synchronization validation
* operational reconstruction validation
* workflow-driven integration testing

UI components, if implemented, are considered temporary validation tooling only.

Frontend completeness is explicitly out of scope for Div3.