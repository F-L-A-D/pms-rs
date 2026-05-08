# PMS-RS Transaction Policy

## Principle

A single operational workflow must execute within a single transaction boundary.

---

## Transaction Owners

Transaction boundaries are owned by:

* usecases
* orchestration services

These components are responsible for:

* begin transaction
* commit
* rollback

---

## Transaction Consumers

Transactions are consumed by:

* repositories
* projection services
* materializers
* rebuild flows

These components must never:

* create transactions
* commit transactions
* rollback transactions

---

## Projection Policy

Projection flows participate in the same transaction boundary as operational mutations.

This guarantees:

* workflow consistency
* projection synchronization
* replay compatibility
* future event sourcing compatibility

---

## Readonly Flows

Readonly flows may:

* rollback explicitly
* rely on automatic rollback via drop

Readonly flows must never commit transactions.

---

## Architectural Constraints

The following are intentionally prohibited unless explicitly redesigned:

* repositories owning transactions
* projections committing transactions
* projections becoming the source of truth
* business logic inside API handlers
* direct projection mutation without rebuildability
* bypassing timeline/event recording for behavioral changes
