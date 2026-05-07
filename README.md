# pms-rs

pms-rs is a hospitality operational platform implemented in Rust.

The system started as a transactional PMS core and is evolving toward a behavioral hospitality operating system.

The platform combines:

- transactional PMS operations
- behavioral reconstruction
- CRM projection foundations
- forecasting-oriented behavioral modeling
- operational decision reproducibility

The architecture explicitly separates:

- operational truth
- append-only behavioral/accounting history
- derived behavioral projections

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

# Current Capabilities

## Reservation & Inventory

Implemented:

- reservation create
- reservation modify
- reservation cancel
- inventory persistence
- transactional rollback consistency

---

## Room & Stay Operations

Implemented:

- room assignment
- occupancy management
- housekeeping lifecycle
- check-in
- check-out

---

## Billing Core

Implemented:

- folio management
- append-only folio entries
- balance derivation
- room charge posting

Billing history preserves accounting reproducibility while operational state remains mutable where necessary.

---

## Guest & Participant Model

Implemented:

- guest aggregate
- guest profile
- participant-authoritative reservation model
- reservation_guest_relations
- accompany guest foundation

Reservation ownership is participant-authoritative rather than direct single-guest ownership.

Reservation aggregates are reconstructed from operational relations.

---

## Behavioral Foundation

Implemented:

- guest timeline
- guest metrics
- participant-aware behavioral propagation

Behavioral layers are projection-oriented and non-authoritative.

Behavioral projections derive from:

- operational truth
- append-only accounting/behavioral history

Examples:

- guest timeline
- guest metrics
- future segmentation
- forecasting features

Behavioral projections may be rebuilt independently from operational state.

---

# Architecture Philosophy

## Operational Truth

Operational truth represents mutable authoritative state.

Examples:

- reservations
- reservation_guest_relations
- rooms
- folios

Operational entities may be:

- modified
- corrected
- reassigned
- cancelled

Operational truth remains authoritative for current-state workflows.

---

## Behavioral History

Behavioral/accounting history preserves reproducibility.

Examples:

- guest timeline
- folio entries

Behavioral history is append-only where operationally meaningful.

Behavioral history supports:

- behavioral reconstruction
- analytics
- CRM interpretation
- forecasting foundations

Behavioral history is not operational authority.

---

## Derived Behavioral Projections

Derived projections support:

- CRM
- segmentation
- forecasting
- personalization
- operational intelligence

Behavioral projections are:

- rebuildable
- disposable
- versionable
- non-authoritative

Projection consistency must never override operational correctness.

---

# Repository Structure

## domain

Core business entities and invariants.

Examples:

- Reservation
- ReservationGuestRelation
- Room
- Folio
- Guest
- GuestTimelineEvent

---

## usecase

Transactional orchestration layer.

Responsibilities:

- transaction boundaries
- operational workflows
- cross-aggregate coordination
- behavioral propagation

---

## repository

Persistence boundary.

Responsibilities:

- SQL access
- persistence isolation
- row mapping

Repositories must not contain business rules.

---

## api

HTTP layer.

Responsibilities:

- request/response mapping
- DTO conversion
- error mapping

---

## tests

Integration-first validation layer.

```bash
cargo test
```

Focus areas:

- transactional consistency
- aggregate reconstruction
- behavioral consistency
- projection correctness

---

# Architectural Principles

The architecture prioritizes:

- operational reproducibility
- behavioral reproducibility
- projection rebuildability
- transactional correctness
- realistic hospitality workflows

The system intentionally avoids:

- premature abstraction
- unnecessary framework-style indirection
- projection-driven operational ownership
- pseudo-event-sourcing authority models

---

# Current Focus

Current architectural focus areas:

- CRM projection boundaries
- segmentation foundation
- behavioral reconstruction consistency
- forecasting-oriented behavioral modeling
- operational intelligence foundation

---

# Long-Term Vision

The long-term goal is a hospitality behavioral operating system capable of:

- CRM
- forecasting
- guest intelligence
- behavioral analytics
- operational decision support
- revenue intelligence
- reproducible operational analysis

The system is designed to evolve incrementally while preserving operational correctness and architectural clarity.