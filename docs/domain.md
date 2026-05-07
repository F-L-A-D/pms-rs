# PMS-RS Domain

## Reservation

### Definition

Reservation represents an operational booking fact.

### Principles

* reservation is operational source-of-truth
* reservation may exist independently from guest identity
* reservation lifecycle must remain transactionally consistent

---

## Room

### Definition

Room represents a physical sellable inventory unit.

### Principles

* room state transitions are explicit
* occupancy and housekeeping are separated concerns

---

## Folio

### Definition

Folio represents a billing container attached to operational stay context.

### Principles

* folio aggregates financial entries
* folio itself does not represent balance

---

## FolioEntry

### Definition

FolioEntry represents append-only billing events.

### Principles

* entries are append-only
* balance is derived from entries
* billing history is immutable

---

## Guest


### Definition

Guest represents a long-lived hospitality identity.

Reservations, stays, billing events, and behavioral projections are linked through guest identity.

### Attributes

* id
* profile information
* contact information

### Behavior

* maintain identity across stays
* aggregate operational history
* provide CRM / behavioral foundation

---

## Guest Aggregate Responsibility

Guest aggregates represent operational guest identity.

Examples:

- guest profile
- membership identifiers
- operational guest linkage

Behavioral interpretation belongs to projection layers rather than operational identity ownership.

Examples:

- segmentation
- loyalty scoring
- behavioral classification
- forecasting features

---

## GuestTimelineEvent

### Definition

GuestTimelineEvent represents behavioral milestones derived from operational events.

### Principles

* timeline is behavioral projection
* timeline is NOT source-of-truth
* operational-only events are excluded
* events are append-only

### Current Events

* ReservationCreated
* ReservationCancelled
* CheckedIn
* CheckedOut
* RoomChargePosted

---

## GuestMetrics

### Definition

GuestMetrics represents derived behavioral projections calculated from operational history.

### Current Metrics

* total_stays
* total_nights
* total_spending
* last_stay_at

### Principles

* metrics are derived on read
* operational tables remain source-of-truth
* timeline is not used as authoritative source