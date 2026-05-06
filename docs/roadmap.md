# PMS-RS Roadmap

## Vision

Build a hospitality operating system centered around:

- reservation lifecycle
- room operations
- billing events
- guest identity
- behavioral history
- future operational and behavioral integrations

The system prioritizes:

- reproducible decision making
- event-oriented design
- extensibility
- operational consistency

---

# Div1 - Operational Core

## Goal

Establish a reproducible and transactionally consistent operational foundation for hotel execution.

Div1 represents the completed operational core layer of the PMS.

---

## M1 - Reservation & Inventory

### Scope

- reservation lifecycle
- inventory persistence
- transaction handling

---

## M2 - Room & Assignment

### Scope

- room management
- occupancy status
- housekeeping status
- room assignment

---

## M3 - Stay Operations

### Scope

- check-in
- check-out
- operational room flow

---

## M4 - Billing Core

### Scope

- folio management
- append-only ledger
- balance derivation
- charge posting
- payment posting

---

## M5 - API Layer

### Scope

- HTTP server
- routing
- serialization
- operational APIs
- integration validation

---

# Div2 - Guest & Behavioral Foundation

## Goal

Build long-lived guest identity and behavioral infrastructure independent from transactional reservation flow.

Reservations, stays, and billing are treated as operational events linked to guest identity.

---

# M6 - Guest Identity Core

## Goal

Implement long-lived guest identity management.

## Scope

- guest entity
- guest profile
- guest persistence
- guest API
- guest search foundation

## Issues

### #16 Guest Aggregate Root

- guest entity
- guest_id
- repository
- persistence

### #17 Guest Profile

- name
- phone
- email
- address
- nationality

### #18 Guest API

- create guest
- update guest
- get guest
- search guest

### #19 Guest Validation

- duplicate prevention foundation
- identity consistency

### #20 Guest Search Foundation

- guest lookup
- search indexing foundation

---

# M7 - Reservation / Guest Relation

## Goal

Connect operational transactions to guest identity.

## Scope

- primary guest
- accompany guest
- reservation linkage
- stay linkage

## Issues

### #21 Primary Guest Relation

### #22 Accompany Guest Support

### #23 Reservation Guest Validation

### #24 Stay / Billing Guest Linkage

---

# M8 - Guest Behavioral Foundation

## Goal

Accumulate operational history as behavioral data.

## Scope

- guest history
- stay history
- spending history
- behavioral metrics

## Issues

### #25 Guest History

### #26 Guest Preference

### #27 Guest Behavioral Metrics

### #28 Guest Timeline API

---

# M9 - CRM Foundation

## Goal

Build hospitality CRM primitives.

## Scope

- membership foundation
- guest segmentation
- corporate relation

## Issues

### #29 Company / Organization

### #30 Membership Foundation

### #31 Guest Segmentation

### #32 CRM Search API

---

# M10 - Guest Intelligence Foundation

## Goal

Build analytical foundation for future forecasting and behavioral modeling.

## Scope

- behavioral event modeling
- guest graph foundation
- forecast linkage foundation

## Issues

### #33 Guest Graph Foundation

### #34 Behavioral Event Model

### #35 Guest Analytics API

### #36 Forecast / CRM Link Foundation

---

# Development Rules

## Scope Management

If implementation requires scope expansion:

1. Propose roadmap modification first
2. Update milestone / issue structure
3. Implement after agreement

## Design Principles

- state and events should be separated where reasonable
- balance is derived from ledger entries
- append-only operations are preferred
- operational consistency is prioritized over premature abstraction
- guest identity and operational events should remain loosely coupled