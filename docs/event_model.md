# Event Model

## Purpose

The event model exists to support:

- behavioral reconstruction
- CRM projection
- segmentation
- forecasting
- operational reproducibility

The event model is NOT an event sourcing authority layer.

Operational truth remains authoritative.

---

# Event Categories

## Operational Events

Operational events represent operational state transitions.

Examples:

- reservation created
- reservation modified
- reservation cancelled
- checked in
- checked out
- room assigned

Operational events originate from mutable operational truth.

Operational events do not replace operational state authority.

---

## Behavioral Events

Behavioral events represent guest-centric semantic interpretation.

Examples:

- guest stayed
- repeat stay
- accompanied stay
- upgraded stay
- high-spending stay
- cancellation behavior

Behavioral events are:

- projection-oriented
- interpretation-driven
- rebuildable
- non-authoritative

Behavioral events may evolve independently from operational schema.

---

# Event Ownership

Operational authority belongs to operational entities.

| Responsibility | Authority |
|---|---|
| reservation state | reservations |
| participant ownership | reservation_guest_relations |
| accounting history | folio_entries |
| room operational state | rooms |
| behavioral interpretation | projections |

Behavioral projections must not become operational authority.

---

# Projection Lifecycle

Behavioral projections are:

- rebuildable
- disposable
- recalculable
- versionable

Projection persistence exists for:

- query performance
- analytics
- CRM
- forecasting

Projection storage must not introduce operational ownership.

---

# Projection Safety Rules

Behavioral projections must not:

- mutate operational truth
- own workflow state
- replace operational authority
- block operational correction

Projection consistency is secondary to operational correctness. 