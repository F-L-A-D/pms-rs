# PMS-RS Roadmap (M4+)

## M4 - Billing Core

### Goal

Implement core billing functionality for stay operations.

Scope is limited to:

* folio management
* billing entries
* balance calculation
* room charge posting

This milestone does NOT include:

* POS integration
* IoT integration
* CRM
* external payment gateway
* analytics

---

#25 - Folio Core

## Summary

Implement folio container for billing operations.

## Scope

* [x] Define `folios` table
* [x] Implement `Folio` entity
* [x] Implement `FolioStatus`
* [x] Implement open / close operations
* [x] Implement repository
* [x] Add integration tests

## Done Criteria

* [x] Folio can be opened
* [x] Folio can be closed
* [x] Closed folio cannot be closed twice

---

#26 - Folio Entry Core

## Summary

Implement billing entry model.

## Scope

* [ ] Define `folio_entries` table
* [ ] Implement `FolioEntry` entity
* [ ] Implement `EntryType`
* [ ] Implement repository
* [ ] Add integration tests

## Entry Types

Initial types:

* RoomCharge
* Payment
* Adjustment

## Done Criteria

* [ ] Billing entries can be stored
* [ ] Entry types are validated
* [ ] Entries belong to a folio

---

#27 - Balance Calculation

## Summary

Implement folio balance calculation.

## Scope

* [ ] Calculate balance from folio entries
* [ ] Implement balance usecase
* [ ] Add integration tests

## Notes

Balance must be derived from entries.

Balance itself is not the source of truth.

## Done Criteria

* [ ] Positive balances are calculated correctly
* [ ] Negative balances are calculated correctly
* [ ] Empty folio returns zero

---

#28 - Room Charge Posting

## Summary

Implement room charge posting flow.

## Scope

* [ ] Post room charge entries
* [ ] Connect stay operations to billing
* [ ] Add integration tests

## Notes

This issue only covers stay-related room charges.

## Done Criteria

* [ ] Room charge entries are created
* [ ] Charges are linked to folios
* [ ] Balance reflects room charges

---

## M5 - API Layer

### Goal

Expose PMS operations through HTTP API.

This milestone introduces:

* HTTP server
* routing
* JSON serialization
* API endpoints

This milestone does NOT include:

* authentication
* authorization
* external integrations
* websocket/event streaming

---

#29 - HTTP Server Setup

## Summary

Setup API server foundation.

## Scope

* [ ] Introduce axum
* [ ] Setup router
* [ ] Setup application state
* [ ] Setup JSON responses
* [ ] Add health check endpoint

## Done Criteria

* [ ] Server boots successfully
* [ ] Health check endpoint responds
* [ ] Shared DB state works

---

#30 - Reservation API

## Summary

Implement reservation API endpoints.

## Scope

* [ ] Create reservation endpoint
* [ ] Modify reservation endpoint
* [ ] Cancel reservation endpoint
* [ ] Add integration tests

## Done Criteria

* [ ] Reservations can be created via HTTP
* [ ] Reservations can be modified via HTTP
* [ ] Reservations can be cancelled via HTTP

---

#44 - Room / Stay API

## Summary

Implement room and stay operation endpoints.

## Scope

* [ ] Assign room endpoint
* [ ] Check-in endpoint
* [ ] Check-out endpoint
* [ ] Housekeeping endpoints
* [ ] Add integration tests

## Done Criteria

* [ ] Room assignment works through API
* [ ] Stay operations work through API
* [ ] Housekeeping lifecycle works through API

---

#45 - Billing API

## Summary

Implement billing-related API endpoints.

## Scope

* [ ] Open folio endpoint
* [ ] Post entry endpoint
* [ ] Balance endpoint
* [ ] Add integration tests

## Done Criteria

* [ ] Folios can be opened via API
* [ ] Entries can be posted via API
* [ ] Balance can be retrieved via API
