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
* Inventory is operational constraint, not pricing logic

## Billing Design

* Billing uses append-only ledger entries
* Balance is derived from entries
* Folio acts as billing container
* Billing events may later be consumed as behavioral signals

## Event Design

* State and events should be separated where reasonable
* Operational history is treated as first-class data
* Behavioral history is treated as analytical foundation

## Guest-Centric Modeling

* Guest is treated as long-lived identity
* Reservations and stays are treated as operational events
* Guest identity and operational events remain loosely coupled

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

* RMS integration
* CRM integration
* guest behavioral analytics
* forecast reproducibility
* cleaning optimization
* IoT (smart lock integration)