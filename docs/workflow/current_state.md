# Current State

## Phase

Projection runtime stabilization is mostly complete.

The current focus is practical hospitality domain expansion:

- operational realism
- workflow reconstruction correctness
- settlement/accounting separation
- semantic clarity
- room housekeeping and occupancy semantics

## Established Projection Guarantees

Preserve these invariants:

- topology-managed propagation
- deterministic traversal ordering
- rebuildable projection chains
- refresh/rebuild equivalence
- transaction-scoped propagation consistency
- projection authority boundary

Projection runtime redesign is not the default task.

## Recent Domain Direction

Billing and settlement now separate operational authority more explicitly:

- folio lifecycle uses `open`, `locked`, and `closed`
- invoices require closed folios
- payments and folio entry mutation require open folios
- settlement transitions remain operational-command oriented

Room operations now separate static room identity from date-scoped state:

- `Room` is static room information
- `RoomDailyState` represents `Room x service_date`
- housekeeping updates mutate `RoomDailyState`
- room API responses do not fabricate occupancy or housekeeping state

## Current Expansion Areas

Good next areas:

- housekeeping operational workflow depth
- stay/check-in integration with room daily occupancy
- room out-of-order and maintenance workflows
- settlement/accounting lifecycle refinement
- API/input/response DTO consistency
- integration tests for realistic hospitality workflows

Areas that should remain stable:

- projection runtime and topology orchestration
- transaction ownership rules
- repository transaction behavior
- explicit enum snake_case persistence
- operational/projection authority separation

## Non-Goals

Do not prioritize:

- frontend completeness
- async propagation
- distributed invalidation
- traversal optimization
- partial rebuild optimization
- incremental projection engine
- framework extraction
