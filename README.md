# PMS-RS

A hotel Property Management System (PMS) core implemented in Rust.

## Overview

This project aims to build a reliable and extensible hospitality backend focusing on:

* reservation lifecycle
* room operations
* billing events
* guest behavioral projection
* transactional consistency
* operational reproducibility

The system is designed with:

* clean architecture
* event-oriented thinking
* append-only billing
* strong transactional guarantees
* timeline-based behavioral foundation

---

## Design Principles

* domain contains core business rules
* usecase orchestrates state transitions
* repository isolates SQL access
* append-only operations are preferred where reasonable
* balance is derived from ledger entries
* operational consistency is prioritized over premature abstraction
* guest identity and operational events remain loosely coupled
* behavioral projections are separated from operational source-of-truth

---

## Behavioral Architecture

The system separates:

* operational facts
* behavioral events
* behavioral projections

### Operational Facts

Operational source-of-truth entities:

* reservations
* folios
* folio_entries

### Behavioral Events

Guest-centric behavioral milestones:

* ReservationCreated
* ReservationCancelled
* CheckedIn
* CheckedOut
* RoomChargePosted

### Behavioral Projections

Derived guest-oriented read models:

* guest timeline
* guest metrics
* future CRM projections

Operational entities remain authoritative while behavioral layers are derived independently for CRM and guest intelligence purposes.

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

# Div2 - Guest & Behavioral Foundation

## M6 - Guest Identity Core

* [x] Guest aggregate
* [x] Guest profile
* [x] Guest API
* [x] Guest search foundation

## M7 - Reservation / Guest Relation

* [x] Primary guest relation
* [ ] Accompany guest support
* [x] Reservation linkage
* [x] Stay linkage
* [x] Billing linkage foundation

## M8 - Guest Behavioral Foundation

* [x] Guest behavioral timeline
* [x] Behavioral projection
* [x] Stay history foundation
* [x] Spending history foundation
* [x] Behavioral metrics

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