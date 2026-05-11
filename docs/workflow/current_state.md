## Projection Invalidation Semantics

Status: FOUNDATION COMPLETE

Implemented:

* `ProjectionInvalidation`
* `ProjectionScope`
* `ProjectionRefreshTarget`
* topology-owned invalidation propagation semantics
* `InvalidationTraversalPlanner`
* `AffectedProjectionSubgraph`
* invalidation-aware orchestrator flow
* dependency-level invalidation policies
* rebuild / invalidation semantic separation
* responsibility-oriented projection test structure

Current semantics:

* propagation remains deterministic
* propagation remains topology-owned
* invalidation derives affected projection subgraph
* refresh execution consumes affected subgraph traversal order
* projections remain rebuildable/disposable/non-authoritative
* operational correctness remains authoritative

Intentional non-features:

* async propagation
* distributed invalidation
* traversal optimization
* partial rebuild execution
* incremental projection engine
* cache-aware propagation pruning

Current architecture direction:

Projection invalidation is now modeled as semantic topology propagation rather than execution-side downstream refresh chaining.

Next target:

## Scoped Rebuild Semantics

Status: FOUNDATION COMPLETE

Implemented:

* authoritative rebuild-equivalence boundary semantics
* affected projection subgraph convergence semantics
* scoped rebuild convergence contracts
* refresh execution as rebuild-equivalence fulfillment
* boundary-scoped correctness semantics
* execution consumption of semantic rebuild boundaries

Current semantics:

* affected projection subgraphs define authoritative rebuild boundaries
* refresh propagation fulfills rebuild-equivalence contracts
* refresh/rebuild symmetry is scoped to affected boundaries
* rebuild equivalence guarantees are boundary-scoped
* execution consumes semantic rebuild boundaries
* topology remains authoritative for boundary derivation
* projections remain rebuildable/disposable/non-authoritative
* operational correctness remains authoritative

Boundary semantics:

* rebuild-equivalence guarantees apply only within affected boundaries
* projection equivalence outside affected boundaries is intentionally undefined
* scoped refresh correctness does not imply global projection convergence

Intentional non-features:

* no partial rebuild optimization
* no incremental rebuild engine
* no asynchronous convergence semantics
* no distributed rebuild semantics
* no cache-aware pruning
* no execution heuristics

Current architecture direction:

Projection invalidation now derives authoritative rebuild-equivalence boundaries rather than execution-oriented refresh sets.

Refresh propagation is modeled as scoped convergence toward authoritative rebuild-equivalent state.

---

## Parallel Exploration: AR Foundation

Exploration in progress:

- BillingAccount
- Invoice
- Receivable
- folio close workflow
- invoice issuance workflow
- operational settlement boundary modeling

Current direction:

Maintain strict separation between:

- operational billing authority
- settlement/accounting authority
- projection intelligence layers