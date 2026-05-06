# PMS-RS Domain Model

## Overview

The domain represents the core operational and behavioral entities of the PMS.

---

## Reservation

### Definition

Reservation represents an operational booking transaction for a future stay.

### Attributes

* id
* check_in
* check_out
* reservation_status (Active / Cancelled)
* stay_status (Confirmed / CheckedIn / CheckedOut)

### Behavior

* Validate date range
* Provide stay nights
* Transition status

---

## Room

### Definition

Room represents a physical unit in the hotel.

### Attributes

* id
* room_class
* occupancy_status (Vacant / Occupied)
* housekeeping_status (Dirty / Cleaning / Cleaned / Inspected)

### Behavior

* Assign to reservation
* Transition occupancy state
* Transition housekeeping lifecycle

---

## Stay

### Definition

Stay represents operational occupancy lifecycle derived from reservation state.

### Behavior

* Check-in
* Check-out
* Occupancy transition

---

## Billing

Billing is modeled as append-only ledger events.

### Components

* Folio
* FolioEntry

### Principles

* Balance is derived
* Entries are append-only
* Billing events are treated as operational history

---

## Guest (Planned)

### Definition

Guest represents a long-lived hospitality identity.

Reservations, stays, and billing events are linked to guests as operational history.

### Attributes

* id
* profile information
* contact information
* guest relations

### Behavior

* Maintain identity across stays
* Aggregate operational history
* Provide behavioral foundation for CRM / analytics

---

## Inventory (Design Note)

Inventory is NOT a domain entity.

* It is a derived and aggregated dataset
* Stored for performance and consistency
* Managed within repository layer
* Represents operational constraints, not pricing decisions

---

## Design Principles

* Domain contains only core business entities
* Derived data must not pollute domain layer
* Usecase orchestrates domain + persistence