# PMS-RS Requirements

## Goal

Build a minimal but consistent PMS backend.

## Scope (Phase 1–5)

* Reservation management
* Inventory management
* Room management
* Stay operations (check-in / check-out)
* Billing
* API layer

## Core Requirements

### Reservation

* Create / modify / cancel reservation
* Reservation has check-in / check-out
* Reservation has status (Active / Cancelled)

### Inventory

* Inventory is managed per date
* Inventory must be consistent with reservation
* Overbooking is allowed
* Negative inventory is not allowed

### Room

* Rooms must exist as entities
* Each reservation is assigned to a room
* Room status must be tracked

### Stay

* Reservation transitions to stay via check-in
* Stay ends via check-out

### Billing

* Billing is modeled as append-only ledger entries
* Balance is derived from ledger entries
* Charges and payments are recorded as events

### API

* All operations must be externally callable

## Non-Goals (for now)

* UI
* Authentication
* Multi-property support
