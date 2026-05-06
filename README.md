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

---

## Current Status

See:

* `roadmap.md`
* `requirements.md`
* `domain.md`
* `decisions.md`
* `testing.md`

### M1 - Reservation & Inventory

* [x] Reservation create
* [x] Reservation modify
* [x] Reservation cancel
* [x] Inventory persistence
* [x] Transaction rollback handling

### M2 - Room & Assignment

* [x] Room entity
* [x] Occupancy status
* [x] Housekeeping status
* [x] Room assignment
* [x] Room class validation

### M3 - Stay Operations

* [x] Check-in
* [x] Check-out
* [x] Housekeeping lifecycle

### M4 - Billing Core

* [x] Folio core
* [x] Folio entry core
* [ ] Balance calculation
* [ ] Room charge posting

### M5 - API Layer

* [ ] HTTP server
* [ ] Reservation API
* [ ] Room / Stay API
* [ ] Billing API

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

---

## Development Workflow

```plaintext
Issue
→ Branch
→ PR
→ Review
→ Merge
```

### Example

```bash
git checkout -b feature/folio-entry-core
```

---

## Testing

Run all tests:

```bash
cargo test
```

### Testing Focus

* state transition validation
* transactional consistency
* repository persistence
* append-only ledger behavior

---

## Setup

```bash
git clone <repo>
cd pms-rs
cargo test
```

---

## Database

### Development

* SQLite

### Planned

* MySQL

---

## Future Considerations

* RMS integration
* operational analytics
* cleaning optimization
* IoT integration
* smart lock support
* event-driven operational modeling
