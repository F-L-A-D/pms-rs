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

---

# ADR: Operational Billing vs Settlement Separation

## Status

Accepted

---

## Context

Hospitality billing workflows contain multiple distinct responsibilities:

* operational stay billing
* settlement ownership
* liability tracking
* accounting finalization

Initially, folios represented both:

* operational billing workflow
* settlement responsibility

This coupling becomes insufficient for:

* corporate billing
* agency billing
* future AR workflows
* settlement tracking
* receivable lifecycle management

Operational billing correction workflows also require continued operational mutability before settlement finalization.

---

## Decision

The architecture explicitly separates:

* Folio
* BillingAccount
* Invoice
* Receivable

### Folio

Folio represents operational billing workflow authority.

Folio remains operationally mutable where correction workflows require it.

### BillingAccount

BillingAccount represents settlement responsibility ownership.

Billing responsibility is independent from:

* guest identity
* reservation participation
* operational reservation ownership

### Invoice

Invoice represents immutable settlement snapshot generation.

Invoice issuance establishes operational/accounting cutoff boundaries.

### Receivable

Receivable represents outstanding settlement liability tracking.

Receivables exist independently from operational folio mutation workflows.

---

## Consequences

Benefits:

* supports future AR workflows
* supports corporate settlement ownership
* preserves operational correction flexibility
* preserves accounting reproducibility
* avoids projection authority leakage
* avoids premature accounting engine abstraction

Tradeoffs:

* lifecycle coordination complexity increases
* operational/accounting boundaries become explicit
* settlement workflows require additional orchestration

These tradeoffs are accepted in favor of operational correctness and future extensibility.

---

# ADR: Settlement Reconstruction Semantics

## Status

Accepted

---

## Context

Operational hospitality billing workflows and accounting settlement semantics are related but not identical.

Examples:

* payment posting may occur before settlement allocation
* invoices may remain unsettled after operational folio closure
* receivables may become disputed independently from operational billing workflows
* operational correction workflows may continue after settlement-related actions occur

The architecture already separates:

* operational billing authority
* settlement/accounting authority
* projection intelligence layers

However, settlement replayability and accounting reconstruction semantics require additional clarification regarding authoritative accounting state.

---

## Decision

The architecture defines settlement/accounting reconstruction semantics as follows.

### Operational Billing Mutations

Operational billing workflows remain operational authorities.

Examples:

* room charge posting
* payment posting
* folio correction
* folio reassignment
* folio closure

Operational mutations represent hospitality workflow actions rather than accounting settlement truth.

Operational workflows remain mutable where operational recovery or correction is required.

---

### Settlement Interpretation Boundary

Accounting settlement semantics are interpreted separately from operational billing mutations.

Examples:

* invoice issuance
* receivable opening
* payment allocation
* settlement reversal
* receivable settlement
* write-off
* dispute transitions

Settlement interpretation represents accounting meaning rather than operational workflow mutation.

This separation exists to preserve replayability and future accounting extensibility.

---

### SettlementTransition Authority

SettlementTransition records represent append-only accounting/settlement transition history.

SettlementTransition history is treated as:

* replayable
* append-oriented
* deterministic
* reconstruction-compatible

SettlementTransition history must not be mutated in-place.

Settlement transitions represent accounting semantic transitions rather than API action logs or projection synchronization events.

---

### Receivable Authority

Receivables represent collectible obligation identity and lifecycle ownership.

Examples:

* settlement responsibility
* invoice-linked obligation tracking
* dispute lifecycle
* write-off lifecycle

Receivables may contain mutable summary state for workflow convenience.

However:

mutable receivable summaries must not become exclusive accounting reconstruction authority.

Outstanding settlement state must remain reconstructable from authoritative settlement transition history.

---

### Derived Accounting Queries

Derived accounting queries are permitted as authoritative operational/accounting queries.

Examples:

* outstanding receivable balance
* settlement exposure
* collectible balance derivation

Derived accounting queries may reconstruct state from:

* settlement transitions
* invoice authority
* operational accounting references

These queries are not projections.

---

### Projection Independence

Projection systems remain:

* rebuildable
* disposable
* non-authoritative

Projection topology evolution must not affect settlement replay semantics or accounting reconstruction correctness.

Accounting reconstruction must remain projection-independent.

---

## Consequences

Benefits:

* preserves deterministic accounting replayability
* preserves operational correction flexibility
* avoids projection-owned accounting authority
* supports future AR workflows
* supports future settlement allocation semantics
* supports forecasting reinterpretation
* preserves accounting reconstruction invariants

Tradeoffs:

* accounting lifecycle semantics become explicit
* settlement interpretation requires additional orchestration
* operational and accounting semantics diverge intentionally
* derived accounting reconstruction logic becomes architecturally important

These tradeoffs are accepted in favor of operational correctness, replayability, and long-term accounting extensibility.