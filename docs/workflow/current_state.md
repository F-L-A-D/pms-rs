# PMS-RS Current State

### Completed

#### Div1 - Operational PMS Core

* Reservation lifecycle
* Room assignment
* Check-in / check-out
* Folio management
* Billing operations
* Housekeeping operations
* Transaction consistency
* Timeline event recording

#### Div2 - CRM / Projection Layer

* Guest timeline aggregation
* Guest summary projection
* Projection materialization
* Projection rebuild flow
* Projection refresh flow
* Projection integration tests
* Transaction ownership unification

### Transaction Boundary Policy

Usecases and orchestration services own transaction boundaries.

Repositories, projection services, materializers, and rebuild flows consume existing transactions only.

This enables:

* single workflow consistency
* projection synchronization
* replay compatibility
* future event sourcing support

### Current Architecture Direction

The system currently consists of:

* mutable operational PMS core
* append-only behavioral/accounting history
* rebuildable projection layer
* projection-oriented CRM foundation

### Next Phase

#### Div3 - Workflow Validation UI

The next phase focuses on validating operational workflows through a lightweight UI layer.

Primary objectives:

* verify workflow consistency
* validate operational usability
* verify projection usefulness
* evaluate search responsiveness
* identify missing query models/projections
* pressure test aggregate boundaries