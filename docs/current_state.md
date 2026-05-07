# pms-rs Current State

## Overview

pms-rs is a hospitality operational platform implemented in Rust.

The system started as a transactional PMS core and is evolving toward a behavioral hospitality platform.

The architecture separates:

- operational facts
- behavioral events
- behavioral projections

Operational consistency is prioritized over premature abstraction.

---

# Completed Milestones

## M1 - Reservation & Inventory

Implemented:

- reservation create
- reservation modify
- reservation cancel
- inventory persistence
- transactional rollback consistency

---

## M2 - Room & Assignment

Implemented:

- room aggregate
- occupancy status
- housekeeping lifecycle
- room assignment

---

## M3 - Stay Operations

Implemented:

- check-in
- check-out
- room operational transitions

---

## M4 - Billing Core

Implemented:

- folio
- folio entries
- append-only billing
- balance derivation
- room charge posting

---

## M5 - API Layer

Implemented:

- Axum HTTP API
- reservation API
- room/stay API
- billing API

---

## M6 - Guest Identity Core

Implemented:

- guest aggregate
- guest profile
- guest CRUD API
- guest search foundation

Guest profile currently supports:

- nationality
- birth_date
- gender
- membership_code
- marketing_opt_in

---

## M7 - Reservation / Guest Relation

Implemented:

- reservation ↔ guest relation
- participant-based reservation model
- primary participant semantics
- accompany guest support foundation
- behavioral guest linkage
- billing linkage foundation

Reservation authority is participant-driven rather than direct guest ownership.

---

## M8 - Guest Behavioral Foundation

Implemented:

### Guest Timeline

Behavioral events:

- ReservationCreated
- ReservationCancelled
- CheckedIn
- CheckedOut
- RoomChargePosted

Timeline is a behavioral projection layer derived from operational truth.
Timeline exists for behavioral reconstruction and guest-centric interpretation.
Timeline is append-only but non-authoritative.
Timeline must not replace operational truth.

### Guest Metrics

Derived projections:

- total_stays
- total_nights
- total_spending
- last_stay_at

Metrics are derived behavioral projections rebuilt from:

- operational tables
- append-only accounting/behavioral history

Metrics are non-authoritative read models.

--- 

## M9 - CRM Foundation

### M9-a - Participant Authority Migration

Implemented:

* participant-authoritative reservation model
* ReservationGuestRelation persistence
* reservation aggregate reconstruction from relations
* participant-based behavioral propagation
* participant-aware guest metrics linkage
* reservation read reconstruction foundation

Operational ownership was migrated from:

* primary_guest_id

to:

* ReservationGuestRelation

Reservation state remains mutable operational truth.

Behavioral history remains append-only through:

* guest timeline
* folio entries

### M9-b - CRM Projection Foundation

Planned:

* event model formalization
* CRM projection boundary refinement
* segmentation foundation
* behavioral projection architecture
* guest behavioral reconstruction consistency
* membership domain direction refinement


---

# Architectural Principles

## Operational Facts

Source-of-truth entities:

- reservations
- reservation_guest_relations
- folios
- folio_entries

---

## Behavioral Events

Guest-centric behavioral milestones.

Timeline is append-only and projection-oriented.

Timeline is NOT source-of-truth.

---

## Behavioral Projections

Derived read models:

- guest timeline
- guest metrics
- future CRM projections

---

## Aggregate Reconstruction

Operational aggregates may be reconstructed from:

- operational tables
- relation tables

Aggregates must preserve authoritative ownership boundaries.

Behavioral projections must not become operational authority.

---

# Layer Structure

## domain

Core business entities and invariants.

Examples:

- Reservation
- ReservationGuestRelation
- Room
- Folio
- Guest
- GuestTimelineEvent

Responsibilities:

- state transitions
- validation
- business invariants

---

## usecase

Transactional orchestration layer.

Responsibilities:

- transaction boundaries
- operational workflows
- cross-aggregate coordination
- behavioral event propagation

---

## repository

Persistence boundary.

Responsibilities:

- SQL access
- row mapping
- persistence isolation

Repositories must not contain business rules.

---

## db

Database connection and transaction management.

---

## adapter

Input normalization and translation layer.

Examples:

- stay_input normalization

---

## api

HTTP layer.

Responsibilities:

- request/response mapping
- DTO conversion
- HTTP error mapping

---

## tests

Integration-first validation layer.

Focus areas:

- transactional consistency
- API flows
- behavioral projection consistency
- aggregate reconstruction consistency

---

# Error Handling Principles

AppError is the application-layer error model.

## Validation

Invalid domain input/state.

---

## Conflict

Operational state conflict.

---

## NotFound

Missing operational entity.

---

## Infrastructure

Persistence/system failure.

---

# Transaction Principles

Transaction boundaries belong to usecases.

Repositories must not own transactions.

Behavioral projections must not become operational source-of-truth.

Append-only semantics should be preserved where operationally meaningful.

Operational state transitions should remain mutable where operational truth requires current-state authority.

---

# Current Repository State

Core layers currently implemented:

- domain
- usecase
- repository
- db
- adapter
- api
- tests

Testing strategy is integration-first.

Current integration test count:

- 16 passing integration tests

---

# Current Focus

Preparing next-stage behavioral architecture refinement.

Potential next steps:

- event model formalization
- CRM projection boundaries
- segmentation foundation
- corporate relation modeling
- forecasting integration
- behavioral reconstruction consistency

---

# Long-Term Direction

The long-term goal is not only PMS functionality, but a hospitality behavioral operating system capable of:

- CRM
- guest intelligence
- forecasting
- behavioral analytics
- operational decision support
- behavioral reproducibility

The system prioritizes:

- operational reproducibility
- behavioral reproducibility
- projection rebuildability
- separation of operational truth and behavioral intelligence

---

# Collaboration Instructions

Think internally in English for precision and consistency.

Respond to the user in Japanese unless explicitly requested otherwise.

Prioritize:

- architectural consistency
- transactional correctness
- operational reproducibility
- realistic PMS constraints
- behavioral modeling separation
- incremental extensibility

Avoid:

- premature abstraction
- unnecessary framework-style indirection
- turning behavioral projections into source-of-truth
- over-engineering without operational justification