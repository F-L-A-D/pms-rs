# pms-rs

A hotel Property Management System (PMS) core implemented in Rust.

## Overview

This project aims to build a reliable and extensible hospitality backend focusing on:

* reservation lifecycle
* room operations
* billing events
* transactional consistency
* operational reproducibility

The system is designed with:

* clean architecture
* event-oriented thinking
* append-only billing
* strong transactional guarantees

---

## Architecture

```plaintext
domain      - core business entities
usecase     - application logic
repository  - persistence abstraction
db          - database connection / transaction
adapter     - input normalization
tests       - integration & transaction tests
```

### Design Principles

* domain contains core business rules
* usecase orchestrates state transitions
* repository isolates SQL access
* append-only operations are preferred where reasonable
* balance is derived from ledger entries
* operational consistency is prioritized over premature abstraction
* guest identity and operational events remain loosely coupled

---

## Current Status

See:

* `roadmap.md`
* `requirements.md`
* `domain.md`
* `decisions.md`
* `testing.md`

---

# Div1 - Operational Core

## M1 - Reservation & Inventory

* [x] Reservation create
* [x] Reservation modify
* [x] Reservation cancel
* [x] Inventory persistence
* [x] Transaction rollback handling

## M2 - Room & Assignment

* [x] Room entity
* [x] Occupancy status
* [x] Housekeeping status
* [x] Room assignment
* [x] Room class validation

## M3 - Stay Operations

* [x] Check-in
* [x] Check-out
* [x] Housekeeping lifecycle

## M4 - Billing Core

* [x] Folio core
* [x] Folio entry core
* [x] Balance calculation
* [x] Room charge posting

## M5 - API Layer

* [x] HTTP server
* [x] Reservation API
* [x] Room / Stay API
* [x] Billing API

---

# Div2 - Guest & Behavioral Foundation (Planned)

## M6 - Guest Identity Core

* [ ] Guest aggregate
* [ ] Guest profile
* [ ] Guest API
* [ ] Guest search foundation

## M7 - Reservation / Guest Relation

* [ ] Primary guest relation
* [ ] Accompany guest support
* [ ] Reservation linkage
* [ ] Stay linkage

## M8 - Guest Behavioral Foundation

* [ ] Stay history
* [ ] Spending history
* [ ] Behavioral metrics
* [ ] Timeline foundation

## M9 - CRM Foundation

* [ ] Membership foundation
* [ ] Guest segmentation
* [ ] Corporate relation

## M10 - Guest Intelligence Foundation

* [ ] Behavioral event modeling
* [ ] Guest graph foundation
* [ ] Forecast linkage foundation

---

## Billing Design

Billing is modeled as append-only ledger entries.

### Core Components

* `Folio`
* `FolioEntry`

### Principles

* entries are append-only
* balance is derived from entries
* operational history is treated as first-class data
* billing events may later be consumed as behavioral signals