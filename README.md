# pms-rs

pms-rs is a hospitality operational platform implemented in Rust.

The system started as a transactional PMS core and is evolving toward a behavioral hospitality operating system.

The platform combines:

* transactional PMS operations
* behavioral reconstruction
* CRM projection foundations
* forecasting-oriented behavioral modeling
* operational decision reproducibility

The architecture explicitly separates:

* operational truth
* append-only behavioral/accounting history
* derived behavioral projections

Operational correctness and reproducibility are prioritized over premature abstraction.

---

# Core Direction

The long-term direction is layered operational intelligence:

Operational Core
↓
Behavioral Reconstruction
↓
CRM / Segmentation
↓
Forecast / Intelligence
↓
Operational Decision Support

Behavioral projections are intentionally non-authoritative.

Operational workflows remain owned by mutable operational aggregates.

---

# Current Stack

## Backend

### Language

* Rust

### Frameworks

* axum
* tokio

### Database

* SQLite
* sqlx

### Testing

* cargo test
* integration-first testing

---

## Planned Frontend Stack

* TypeScript

The frontend layer is currently planned as a lightweight workflow validation UI for operational testing.

---

## Planned Database Direction

The long-term operational database target is:

* MySQL

SQLite is currently used for:

* rapid iteration
* transactional workflow validation
* integration-focused development

The architecture is intentionally designed to minimize future database migration costs.

---

## Architectural Style

* transactional operational core
* append-only behavioral history
* rebuildable projection layer
* orchestration-oriented usecases

---

# Transaction Boundaries

Transaction boundaries are owned by:

* usecases
* orchestration services

Repositories and projection components consume existing transactions only.

This guarantees:

* workflow consistency
* projection synchronization
* replay compatibility

Projection components must never own transactional consistency.

---

# Current Capabilities

## Reservation & Inventory

Implemented:

* reservation create
* reservation modify
* reservation cancel
* inventory persistence
* transactional rollback consistency

---

## Room & Stay Operations

Implemented:

* room assignment
* occupancy management
* housekeeping lifecycle
* check-in
* check-out

---

## Billing Core

Implemented:

* folio management
* append-only folio entries
* balance derivation
* room charge posting

Billing history preserves accounting reproducibility while operational state remains mutable where necessary.

---

## Guest & Participant Model

Implemented:

* guest aggregate
* guest profile
* participant-authoritative reservation model
* reservation_guest_relations
* accompany guest foundation

Reservation ownership is participant-authoritative rather than direct single-guest ownership.

Reservation aggregates are reconstructed from operational relations.

---

## Behavioral Foundation

Implemented:

* guest timeline
* guest metrics
* guest summary projection
* participant-aware behavioral propagation
* projection rebuild flow

Behavioral layers are projection-oriented and non-authoritative.

Behavioral projections derive from:

* operational truth
* append-only accounting/behavioral history

Examples:

* guest timeline
* guest metrics
* guest summary projection
* future segmentation
* forecasting features

Behavioral projections are intentionally rebuildable from authoritative operational and behavioral history.

Projection loss must not compromise operational correctness.

Behavioral projections may be rebuilt independently from operational state.

---

# Architecture Philosophy

## Operational Truth

Operational truth represents mutable authoritative state.

Examples:

* reservations
* reservation_guest_relations
* rooms
* folios

Operational entities may be:

* modified
* corrected
* reassigned
* cancelled

Operational truth remains authoritative for current-state workflows.

---

## Behavioral History

Behavioral/accounting history preserves reproducibility.

Examples:

* guest timeline
* folio entries

Behavioral history is append-only where operationally meaningful.

Behavioral history supports:

* behavioral reconstruction
* analytics
* CRM interpretation
* forecasting foundations

Behavioral history is not operational authority.

---

## Derived Behavioral Projections

Derived projections support:

* CRM
* segmentation
* forecasting
* personalization
* operational intelligence

Behavioral projections are:

* rebuildable
* disposable
* versionable
* non-authoritative

Projection consistency must never override operational correctness.

---

# Repository Structure

## domain

Core business entities and invariants.

Examples:

* Reservation
* ReservationGuestRelation
* Room
* Folio
* Guest
* GuestTimelineEvent

---

## usecase

Transactional orchestration layer.

Responsibilities:

* transaction boundaries
* operational workflows
* cross-aggregate coordination
* behavioral propagation

---

## repository

Persistence boundary.

Responsibilities:

* SQL access
* persistence isolation
* row mapping

Repositories must not contain business rules.

---

## projection

Rebuildable query model layer.

Responsibilities:

* projection materialization
* projection refresh
* projection rebuild
* operational query optimization

Projections must not:

* own transactions
* become operational authority
* bypass behavioral history

---

## api

HTTP layer.

Responsibilities:

* request/response mapping
* DTO conversion
* error mapping

---

## tests

Integration-first validation layer.

```bash
cargo test
```

Focus areas:

* transactional consistency
* aggregate reconstruction
* behavioral consistency
* projection correctness
* rebuild correctness

---

# Architectural Principles

The architecture prioritizes:

* operational reproducibility
* behavioral reproducibility
* projection rebuildability
* transactional correctness
* realistic hospitality workflows

The system intentionally avoids:

* premature abstraction
* unnecessary framework-style indirection
* projection-driven operational ownership
* pseudo-event-sourcing authority models

---

# Current Focus

Current architectural focus areas:

* workflow validation
* query hardening
* CRM projection boundaries
* segmentation foundation
* behavioral reconstruction consistency
* forecasting-oriented behavioral modeling
* operational intelligence foundation

---

# Div3 - Workflow Validation & Query Hardening

The current phase focuses on workflow-level validation rather than frontend completeness.

Primary objectives:

* verify workflow consistency
* validate projection synchronization
* validate realistic operational search behavior
* verify projection usefulness
* identify missing query models/projections
* pressure test aggregate boundaries
* validate workflow reconstruction consistency

Current validation targets:

* guest workflow validation
* reservation workflow validation
* billing workflow validation
* projection rebuild/refresh equivalence
* UUID-based identity propagation
* aggregate boundary validation
* search precision validation
* false-positive suppression

UI tooling may be introduced temporarily for validation purposes, but frontend implementation is not considered a primary architectural objective at this stage.

The project intentionally prioritizes:

* operational correctness
* workflow reproducibility
* projection consistency
* realistic integration behavior

over frontend completeness.

---

# Long-Term Vision

The long-term goal is a hospitality behavioral operating system capable of:

* CRM
* forecasting
* guest intelligence
* behavioral analytics
* operational decision support
* revenue intelligence
* reproducible operational analysis

The system is designed to evolve incrementally while preserving operational correctness and architectural clarity.

Before implementing new features, review:

* docs/architecture/*
* docs/workflow/*
* docs/core/*