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

Focus areas:

* refresh target semantics formalization
* scope-aware rebuild compatibility
* refresh / rebuild symmetry rules
* authoritative rebuild compatibility with scoped invalidation
* refresh boundary semantics

Constraints:

* rebuild correctness over optimization
* no projection authority leakage
* topology/orchestrator owns propagation semantics
* services remain local-only
* no partial rebuild optimization yet
* no async/distributed consistency concerns yet

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