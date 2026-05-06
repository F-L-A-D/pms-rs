# PMS-RS Domain Model

## Overview

The domain represents the core business entities of the PMS.

## Reservation

### Definition

Reservation represents a booking request for a stay.

### Attributes

* id
* check_in
* check_out
* reservation_status (Active / Cancelled)
* stay_status (Confirmed / CheckedIn / CheckedOut)

### Behavior

* Validate date range
* Provide stay nights
* Transition status (Active → Cancelled)

---

## Room (Planned)

### Definition

Room represents a physical unit in the hotel.

### Attributes

* id
* room_type
* status (Vacant / Occupied / Dirty)

### Behavior

* Assign to reservation
* Track availability

---

## Stay (Planned)

### Definition

Stay represents an actual guest occupancy derived from a reservation.

### Behavior

* Check-in (Reservation → Stay)
* Check-out (Stay ends)

---

### Billing

Billing is modeled as append-only ledger events.

#### Components

* Folio
* FolioEntry

#### Principles

* Balance is derived
* Entries are append-only

---

## Inventory (Design Note)

Inventory is NOT a domain entity.

* It is a derived and aggregated dataset
* Stored for performance and consistency
* Managed within repository layer

---

## Design Principles

* Domain contains only core business entities
* Derived data must not pollute domain layer
* Usecase orchestrates domain + persistence