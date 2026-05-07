# PMS -RS Decisions

## Append-Only Billing

Billing is modeled through append-only ledger entries.

### Rationale

* preserve operational history
* support auditability
* support future behavioral analytics

---

## Operational Consistency First

Operational consistency is prioritized over premature abstraction.

### Rationale

Hotel PMS operations require strong transactional guarantees across:

* reservations
* room assignment
* stay operations
* billing

---

## Guest Identity Separation

Guest identity remains separated from operational facts.

### Rationale

Operational events may exist independently from long-lived guest identity.

This prevents unnecessary coupling between:

* operational lifecycle
* CRM concerns
* behavioral modeling

---

## Behavioral Projection Separation

Operational facts, behavioral events, and behavioral projections are treated as separate layers.

### Operational Facts

Source-of-truth operational entities:

* reservations
* folios
* folio_entries

### Behavioral Events

Guest-centric behavioral projections:

* ReservationCreated
* ReservationCancelled
* CheckedIn
* CheckedOut
* RoomChargePosted

### Behavioral Projections

Derived guest metrics and CRM-oriented read models.

### Rationale

This separation prevents operational consistency concerns from being polluted by analytics concerns while preserving extensibility toward CRM and guest intelligence layers.