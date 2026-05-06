# PMS-RS Roadmap

## Vision

Build a hospitality operating system centered around:
- reservation lifecycle
- room operations
- billing events
- future operational integrations

The system prioritizes:
- reproducible decision making
- event-oriented design
- extensibility
- operational consistency

---

# M1 - Reservation & Inventory

## Goal

Implement reservation lifecycle and inventory control.

## Scope

- reservation creation
- reservation modification
- cancellation
- inventory adjustment
- transaction handling

## Issues

### #1 Reservation Create

- reservation entity
- create usecase
- inventory increment
- persistence

### #2 Reservation Modify / Cancel

- modify reservation
- cancel reservation
- inventory rollback
- transaction rollback tests

---

# M2 - Room & Assignment

## Goal

Implement physical room management.

## Scope

- room entity
- occupancy status
- housekeeping status
- room assignment

## Issues

### #3 Room Core

- room entity
- room_class
- occupancy status
- housekeeping status
- persistence

### #4 Assign Room

- assign reservation to room
- room_class validation
- room status update

---

# M3 - Stay Operations

## Goal

Implement stay lifecycle and operational room flow.

## Scope

- check-in
- check-out
- housekeeping lifecycle

## Issues

### #5 Check-in

- stay status update
- occupancy update
- validation

### #6 Check-out

- checkout processing
- room vacancy update

### #7 Housekeeping Lifecycle

- dirty
- cleaning
- cleaned
- inspected

---

# M4 - Billing Core

## Goal

Implement append-only billing foundation for stay operations.

Billing is modeled as append-only ledger entries.

## Scope

- folio management
- billing entries
- balance calculation
- room charge posting

## Issues

### #8 Folio Core

- folio entity
- folio lifecycle
- repository
- tests

### #9 Folio Entry Core

- folio_entries table
- FolioEntry entity
- EntryType
- repository
- tests

### #10 Balance Calculation

- calculate balance from entries
- balance projection
- tests

### #11 Room Charge Posting

- connect stay operations to billing
- room charge entries
- folio linkage

---

# M5 - API Layer

## Goal

Expose PMS functionality through HTTP API.

## Scope

- HTTP server
- routing
- JSON serialization
- API endpoints

## Issues

### #12 HTTP Server Setup

- axum setup
- router
- app state
- health endpoint

### #13 Reservation API

- create reservation endpoint
- modify reservation endpoint
- cancel reservation endpoint

### #14 Room / Stay API

- assign room endpoint
- check-in endpoint
- check-out endpoint
- housekeeping endpoints

### #15 Billing API

- folio endpoint
- entry posting endpoint
- balance endpoint

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
- internal domain language is English-based
