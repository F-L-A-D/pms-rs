# PMS-RS Architectural Decisions

---

# ADR: Participant-Authoritative Reservation Ownership

## Status

Accepted

---

## Context

Reservation ownership originally depended on:

- primary_guest_id

This structure was insufficient for hospitality operations because reservations may contain:

- multiple participants
- accompanying guests
- family stays
- corporate stays
- evolving participant relationships

Direct reservation ownership by a single guest produced ambiguity in:

- behavioral linkage
- billing association
- guest reconstruction
- future CRM segmentation

---

## Decision

Reservation ownership is participant-authoritative.

Operational ownership is represented through:

- reservation_guest_relations

Reservation aggregates are reconstructed from participant relations rather than direct guest ownership.

The reservation aggregate remains mutable operational truth.

Behavioral propagation derives from participant relations.

---

## Consequences

Benefits:

- supports multi-participant reservations
- supports future CRM evolution
- improves behavioral linkage consistency
- preserves operational flexibility

Tradeoffs:

- aggregate reconstruction complexity increases
- relation consistency becomes architecturally important

These tradeoffs are accepted in favor of operational correctness and extensibility.

---

# ADR: Projection Non-Authority Principle

## Status

Accepted

---

## Context

The system is evolving beyond transactional PMS functionality toward:

- behavioral reconstruction
- CRM
- segmentation
- forecasting
- operational intelligence

As behavioral projections expand, there is risk that:

- projections become operational authority
- timeline becomes pseudo-event-sourcing
- derived metrics become transactional truth

Hospitality operations require mutable operational correction, including:

- reservation reassignment
- room reassignment
- guest merge
- billing correction
- operational recovery

Pure append-only authority models are not suitable for all operational workflows.

---

## Decision

The architecture explicitly separates:

- operational truth
- append-only behavioral/accounting history
- derived behavioral projections

Operational truth remains authoritative.

Behavioral projections remain:

- derived
- rebuildable
- disposable
- non-authoritative

Append-only history does not imply operational ownership.

Projection consistency must not override operational correctness.

---

## Operational Authority

Operational authority belongs to mutable operational aggregates.

Examples:

- reservations
- reservation_guest_relations
- rooms
- folios

Operational aggregates may be corrected, reassigned, or updated.

Operational truth is authoritative for current-state workflows.

---

## Behavioral History

Behavioral/accounting history is append-only where operationally meaningful.

Examples:

- guest timeline
- folio entries

Behavioral history supports:

- behavioral reconstruction
- reproducibility
- analytics
- CRM interpretation

Behavioral history must not replace operational truth.

---

## Projection Principles

CRM and behavioral projections:

- derive from operational truth and behavioral history
- support read-oriented intelligence
- may be recalculated or rebuilt
- may evolve independently from operational schema

Examples:

- guest metrics
- segmentation
- forecasting features
- behavioral analytics

Projection persistence exists for efficiency and intelligence, not authority.

---

## Consequences

Benefits:

- preserves operational correctness
- supports mutable hospitality workflows
- enables behavioral intelligence evolution
- avoids projection authority leakage
- avoids pseudo-event-sourcing complexity

Tradeoffs:

- projections may temporarily diverge from operational truth
- rebuild pipelines become architecturally important
- behavioral consistency becomes eventual rather than transactional

These tradeoffs are accepted in favor of operational reproducibility and realistic hospitality workflows.