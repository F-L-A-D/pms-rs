# PMS-RS Requirements

## Goal

Build a consistent hospitality operational backend with reproducible transactional behavior.

---

# Scope (Div1 - Operational Core)

* Reservation management
* Inventory management
* Room management
* Stay operations
* Billing
* API layer

---

# Planned Scope (Div2 - Guest & Behavioral Foundation)

* Guest identity management
* Reservation-guest relation
* Guest behavioral history
* CRM foundation
* Behavioral analytics foundation

---

# Core Requirements

## Reservation

* Create / modify / cancel reservation
* Reservation has check-in / check-out
* Reservation has status

## Inventory

* Inventory is managed per date
* Inventory must remain consistent
* Overbooking is allowed
* Negative inventory is not allowed
* Inventory is treated as operational constraint
* Pricing and overbooking logic are externalized

## Room

* Rooms must exist as entities
* Reservations are assigned to rooms
* Occupancy status and housekeeping lifecycle are tracked independently

## Stay

* Reservation transitions to stay via check-in
* Stay ends via check-out

## Billing

* Billing is modeled as append-only ledger entries
* Balance is derived from ledger entries
* Charges and payments are recorded as events
* Billing history is treated as operational event history

## API

* All operations must be externally callable

---

## Non-Goals (for now)

* UI
* Authentication
* Multi-property support
* Pricing strategy
* Forecasting logic
* RMS optimization