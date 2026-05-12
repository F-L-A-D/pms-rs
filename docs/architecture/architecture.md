# PMS-RS Operational Truth vs Behavioral Projection

The architecture explicitly separates:

- operational truth
- behavioral history
- derived intelligence

## Operational Truth

Mutable operational entities representing current authoritative state.

Examples:

- reservations
- reservation_guest_relations
- rooms
- folios

Operational truth is authoritative for:

- reservation ownership
- occupancy state
- room assignment
- billing responsibility
- operational workflows

Operational truth may be:

- modified
- reassigned
- corrected
- cancelled

Operational truth must remain mutable where operational reality requires correction.

---

## Behavioral History

Append-only behavioral/accounting records.

Examples:

- guest timeline
- folio entries

Behavioral history preserves reproducibility of:

- guest behavior
- operational actions
- accounting transitions

Behavioral history is projection-oriented and non-authoritative.

Behavioral history must not replace operational truth.

---

## Behavioral Projections

Derived read models generated from:

- operational truth
- behavioral history

Examples:

- guest metrics
- CRM projections
- segmentation
- forecasting features
- behavioral analytics

Behavioral projections are:

- rebuildable
- disposable
- versionable
- non-authoritative

Projection inconsistency must never corrupt operational truth.

---

# Projection Rebuildability

CRM and behavioral projections must be rebuildable from:

- operational tables
- append-only behavioral history

Projection persistence exists for:

- query efficiency
- analytics
- segmentation
- forecasting
- personalization

Projection storage must not introduce independent business authority.

Projection rebuildability is required for:

- schema evolution
- metric refinement
- behavioral reinterpretation
- segmentation redesign
- forecasting improvements

---

# Aggregate Reconstruction

Operational aggregates may be reconstructed from:

- operational tables
- relation tables

Examples:

- reservation_guest_relations
- room assignment relations
- future corporate relations

Aggregate reconstruction must preserve authoritative ownership boundaries.

Behavioral projections must not become reconstruction authority.

Projection-derived state must never replace operational truth during aggregate reconstruction.

---

# Behavioral Projection Constraints

Behavioral projections exist to support:

- CRM
- analytics
- forecasting
- segmentation
- personalization
- operational intelligence

Behavioral projections must not:

- own operational workflows
- mutate operational authority
- become transactional truth
- replace mutable operational state

Projection inconsistency must not block:

- operational correction
- reservation reassignment
- room reassignment
- guest merge
- billing correction

Operational correctness has priority over projection consistency.

---

# CRM Projection Direction

CRM projections exist to support:

- behavioral understanding
- personalization
- segmentation
- forecasting
- revenue intelligence
- operational decision support

CRM projections derive from:

- operational truth
- behavioral history

CRM projections are read-oriented behavioral models.

CRM projections must not directly own:

- reservation authority
- billing authority
- room authority
- operational workflow state

CRM projections are intelligence layers rather than operational transaction layers.

---

## Transaction Ownership Policy

Usecases and orchestration services own transaction boundaries.

Repositories, projections, materializers, and rebuild flows consume existing transactions only.

This policy exists to guarantee:

* single workflow consistency
* projection synchronization
* replay compatibility
* future event sourcing compatibility

---

## Projection Philosophy

Projections are rebuildable query models.

Projections:

* may be deleted and rebuilt
* are not the source of truth
* exist for operational query optimization
* exist for workflow support

Operational entities and append-only history remain authoritative.

---

## Projection Dependency Principles

Projection dependencies are architecture primitives.

Projection dependency graphs must remain:

* explicit
* deterministic
* rebuildable
* topology-managed
* orchestration-owned

Projection dependency traversal must never rely on implicit service-to-service propagation.

---

## Projection Topology

Projection chains are modeled explicitly through topology definitions.

Example:

reservation
→ inventory projection
→ hotel inventory projection

Traversal ordering is centrally managed through topology/orchestrator layers.

---

## Projection Refresh Semantics

Refresh propagation represents incremental downstream synchronization.

Refresh traversal is downstream-only.

Example:

InventoryProjection refresh
→ HotelInventoryProjection refresh

The originating projection is assumed already updated.

---

## Projection Rebuild Semantics

Rebuild traversal represents full reconstruction from operational authority.

Rebuild traversal includes:

* self rebuild
* downstream rebuild

Example:

InventoryProjection rebuild
→ HotelInventoryProjection rebuild

Rebuilds always originate from operational source-of-truth state.

---

## Refresh/Rebuild Symmetry

Incremental propagation and full rebuild traversal must converge to identical projection state.

This is treated as an architectural invariant.

Equivalent operational state must always produce equivalent projection chain state.

---

## Projection Authority Boundary

Projections are not operational authorities.

Downstream projections must never become canonical operational state.

Operational entities remain authoritative.

Projection layers are treated as:

* derived
* disposable
* rebuildable
* query-oriented

---

## Projection Orchestration Ownership

Projection dependency propagation is owned by topology/orchestrator layers.

Projection services should only manage:

* local projection refresh
* local projection materialization
* local projection persistence

Services must not directly own downstream dependency propagation semantics.

Projection-local services must not derive or mutate downstream convergence semantics.

---

## Projection Consistency Guarantees

The architecture guarantees:

* deterministic projection reconstruction
* rebuildable projection chains
* topology-consistent traversal ordering
* refresh/rebuild equivalence
* transaction-scoped propagation consistency

---

## Architectural Constraints

The following are intentionally prohibited unless explicitly redesigned:

* repositories owning transactions
* projections committing transactions
* projections becoming the source of truth
* business logic inside API handlers
* direct projection mutation without rebuildability
* bypassing timeline/event recording for behavioral changes

---

## Operational Billing vs Settlement Responsibility

The architecture explicitly separates:

* operational billing activity
* settlement responsibility
* outstanding liability tracking

These concerns must not collapse into a single authority model.

---

## Folio

Folio represents an operational billing container.

Folios exist to support:

* operational stay billing
* in-stay charge aggregation
* operational payment handling
* mutable operational correction workflows

Folios are operational authorities.

Folios may remain mutable while operational correction remains necessary.

Examples:

* charge correction
* operational adjustment
* reassignment
* operational recovery

Folio balance is derived from append-only folio entries.

---

## BillingAccount

BillingAccount represents settlement responsibility ownership.

Billing responsibility is explicitly separated from:

* guest identity
* reservation participation
* operational stay ownership

This separation exists to support:

* corporate billing
* agency billing
* future split liability workflows
* future external settlement ownership

BillingAccount assignment does not transfer operational reservation ownership.

---

## Invoice

Invoice represents settlement snapshot generation.

Invoices are generated from operational folio state.

Invoice issuance represents:

* settlement finalization boundary
* operational cutoff transition
* accounting snapshot generation

Invoices are immutable settlement artifacts.

Invoices are not operational billing containers.

Invoice issuance must not depend on projection authority.

---

## Receivable

Receivable represents outstanding settlement liability.

Receivables exist independently from folio operational workflows.

Receivables support:

* AR tracking
* settlement workflows
* aging analysis
* payment reconciliation
* future credit workflows

Receivables are operational/accounting authorities.

Receivables are not projections.

---

## Settlement Authority Boundary

The architecture separates:

* operational billing authority
* settlement/accounting authority
* behavioral/analytical projection layers

Projection layers must never become settlement authority.

Examples of non-authoritative projection usage:

* AR aging analytics
* payment risk analysis
* corporate spend analysis
* forecasting
* segmentation

Operational/accounting correctness remains authoritative over projection consistency.

---

## Transactional Billing Policy

Operational settlement workflows own transaction boundaries.

Examples:

* folio closing
* invoice issuance
* receivable generation

Repositories and derived query helpers consume existing transactions only.

Read-oriented convenience queries may internally own local read transactions where operational consistency boundaries are not being coordinated.

---

## Derived Financial Queries

Some financial queries are operationally derived but remain authoritative.

Examples:

* current folio balance
* outstanding receivable balance
* credit exposure

These queries are not projections.

These queries derive authoritative operational/accounting state from operational records within transaction boundaries.

---

## Settlement Reconstruction

Settlement/accounting state must remain reconstructable independently from projections.

Settlement reconstruction derives authoritative accounting state from:

* settlement transition history
* invoice authority
* receivable lifecycle authority
* operational accounting references

Projection systems must not participate in authoritative settlement reconstruction.

---

### Operational Mutation vs Accounting Interpretation

Operational billing mutations and accounting settlement interpretation are intentionally separated.

Examples of operational mutations:

* payment posting
* room charge posting
* folio correction
* folio closure

Examples of accounting interpretation:

* receivable opening
* payment allocation
* receivable settlement
* settlement reversal
* dispute lifecycle transitions
* write-off lifecycle transitions

Operational mutations represent hospitality workflow activity.

Accounting interpretation represents settlement/accounting meaning.

These concerns must not collapse into a single authority model.

---

### SettlementTransition History

SettlementTransition records represent append-only accounting semantic history.

Settlement transition history supports:

* deterministic replay
* accounting reconstruction
* auditability
* future settlement reinterpretation
* forecasting compatibility

Settlement transitions are not:

* projections
* workflow UI events
* downstream synchronization triggers
* mutable accounting summaries

Settlement transitions must remain append-oriented.

---

### Receivable Reconstruction

Receivable lifecycle state must remain reconstructable from authoritative settlement history.

Receivables may contain mutable workflow-oriented summary state.

However:

mutable summaries must not become exclusive accounting reconstruction authority.

Outstanding settlement state must remain reproducible from authoritative accounting history.

---

### Derived Accounting Queries

Derived accounting queries may reconstruct authoritative accounting state from append-oriented accounting history.

Examples:

* outstanding receivable balance
* collectible exposure
* settlement aging
* payment allocation state

These queries are authoritative operational/accounting queries rather than projections.

---

### Projection Independence

Projection rebuilds, topology evolution, invalidation semantics, and projection reconstruction must remain independent from accounting replay semantics.

Projection evolution must never alter authoritative settlement reconstruction results.

---

## Scoped Rebuild Semantics

Affected projection subgraphs define authoritative rebuild-equivalence boundaries.

Refresh propagation fulfills rebuild-equivalence contracts within affected boundaries rather than guaranteeing global projection convergence.

Refresh/rebuild symmetry is treated as an architectural invariant.

Equivalent operational state must converge to equivalent projection state within affected rebuild boundaries.

Projection equivalence guarantees are intentionally boundary-scoped.

Projection state outside affected rebuild boundaries is intentionally outside scoped convergence guarantees.

This separation exists to support future:

* partial rebuild execution
* selective replay
* asynchronous propagation
* distributed rebuild orchestration
* topology-aware execution optimization

without introducing projection authority leakage.

Projection convergence correctness must never override operational correctness requirements.

---

## Affected Projection Boundaries

Affected projection subgraphs represent semantic rebuild-equivalence boundaries rather than execution-oriented projection selection sets.

Affected boundaries are topology-derived and orchestration-owned.

Projection services must not determine downstream rebuild responsibility.

Topology layers remain authoritative for:

* dependency traversal
* boundary derivation
* propagation semantics
* rebuild-equivalence scope determination

Failed projection nodes are intentionally excluded from authoritative convergence completion semantics.

Execution layers consume topology-derived convergence semantics but must not redefine authoritative rebuild boundaries.

---

## Refresh/Rebuild Symmetry

Refresh propagation and authoritative rebuild execution must converge to equivalent projection state within affected rebuild boundaries.

This is treated as a core architectural invariant.

Refresh execution fulfills rebuild-equivalence contracts derived from topology-managed affected projection boundaries.

---

## Boundary-Scoped Correctness

Projection correctness guarantees are scoped to affected rebuild boundaries only.

Scoped refresh correctness intentionally does not imply:

* topology-wide projection convergence
* global projection synchronization
* full rebuild equivalence outside affected boundaries

Boundary-external projection state is intentionally outside scoped equivalence guarantees until affected by future invalidation or rebuild flows.

Partially converged projection state must never become authoritatively visible.

Authoritative visibility emerges only after affected rebuild boundary convergence fulfillment.

Convergence degradation remains isolated to affected rebuild boundaries.

---

# Usecase Layer Semantics

The usecase layer is divided by workflow semantics.

## command

`usecase/*/command`

Command usecases own operational workflow execution.

Responsibilities:

* aggregate state transition
* validation orchestration
* operational persistence
* behavioral append

Allowed:

* `repository.find_by_id`
* `repository.save`
* `repository.modify`
* behavioral repositories
* orchestrators

Disallowed:

* direct projection access
* projection services
* projection repositories
* projection rebuild ownership

Projection rebuilds are orchestrator-owned.


---

## detail

`usecase/*/detail`

Detail usecases provide direct operational entity retrieval.

Responsibilities:

* single aggregate retrieval
* detail screen retrieval
* operational truth access

Allowed:

* `repository.find_by_id`

Disallowed:

* list/search retrieval
* projection access
* behavioral traversal
* aggregate scanning


---

## search

`usecase/*/search`

Search usecases provide list-oriented retrieval.

Responsibilities:

* search retrieval
* keyword filtering
* list queries
* projection-backed query orchestration

Allowed:

* repository list/search queries
* behavioral repositories
* projection services

Disallowed:

* direct projection repository access

Projection access must follow:

usecase
→ projection/services
→ projection/*


---

## calculation

`usecase/*/calculation`

Calculation usecases provide derived computation logic.

Examples:

* balance calculation
* receivable derivation
* settlement computation

Allowed:

* repositories
* behavioral repositories

Disallowed:

* operational mutation
* projection ownership


---

# Projection Ownership

Projection ownership belongs exclusively to the projection layer.

Usecases must not directly own:

* projection persistence
* projection rebuild topology
* projection invalidation traversal

Projection consistency is orchestrator-owned.


---

# Operational Truth Rule

Operational repositories remain authoritative.

Operational entities may be directly retrieved when:

* accessing by identity
* performing transactional workflow
* retrieving detail views

Projection models remain:

* rebuildable
* disposable
* non-authoritative