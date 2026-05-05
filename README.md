# pms-rs

A hotel Property Management System (PMS) core implemented in Rust.

## Overview

This project aims to build a reliable and extensible PMS focusing on:

* Reservation management
* Inventory consistency
* Transactional integrity

## Architecture

* `domain` — business entities (reservation, inventory)
* `usecase` — application logic
* `repository` — data access layer
* `db` — database connection & transaction
* `adapter` — input normalization

## Current Status

* Reservation management: implemented
* Database persistence (SQLite): implemented
* Transaction management: implemented
* Inventory: in-memory (next step)

## Development Workflow

```plaintext
Issue → Branch → PR → Review → Merge
```

### Example

```bash
git checkout -b feature/inventory-db
```

## Testing

```bash
cargo test
```

## Roadmap

### M1: Reservation & Inventory

* [ ] Inventory DB (#1)
* [ ] Reservation + Inventory (#2)

### M2: Room & Status

* [ ] Room entity (#3)
* [ ] Room status (#4)
* [ ] Room assignment (#5)

### M3: Stay Operations

* [ ] Check-in (#6)
* [ ] Check-out (#7)
* [ ] Stay lifecycle (#8)

### M4: Billing

* [ ] Billing schema (#9)
* [ ] Charges (#10)
* [ ] Payments (#11)

### M5: API

* [ ] API layer (#12)
* [ ] Reservation endpoints (#13)
* [ ] Stay endpoints (#14)

## Setup

```bash
git clone <repo>
cd pms-rs
cargo test
```

## Notes

* SQLite is used for development
* MySQL planned for production
