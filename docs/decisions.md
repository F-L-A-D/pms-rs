# PMS-RS Decisions

## Architecture

* Clean Architecture style
* domain / usecase / repository separation

## Language

* Rust for core logic (safety & correctness)

## Database

* SQLite (development)
* MySQL planned for production

## Transaction

* All write operations must be transactional
* Usecase layer controls transaction boundary

## Inventory Design

* Inventory stored as daily aggregated table
* Inventory is NOT derived on the fly
* Inventory is updated alongside reservation

## Billing Design

* Billing uses append-only ledger entries
* Balance is derived from entries
* Folio acts as billing container

## Event Design

* State and events should be separated where reasonable
* Operational history is treated as first-class data

## Overbooking

* Allowed by design
* Controlled externally (RMS)

## Validation Rules

* Negative inventory is forbidden
* Reservation must remain consistent

## Repository Pattern

* DB access isolated in repository layer
* Usecase does not contain SQL

## Future Considerations

* RMS integration (pricing / optimization)
* Cleaning optimization
* IoT (smart lock integration)
