# Review Checklist

Use this checklist before handing off substantial changes.

## Authority

- Operational truth remains authoritative.
- Projections remain derived, rebuildable, disposable, and non-authoritative.
- Settlement authority is not delegated to projections or convenience views.
- Behavioral history is not used as current operational truth.

## Transactions

- Usecases or orchestration layers own transaction boundaries.
- Repositories do not begin, commit, or roll back transactions.
- Projection components consume existing transactions.

## Usecase Shape

- Usecase files are placed under `command`, `detail`, or `search`.
- Usecase file names match usecase names.
- Usecase callable functions are named `execute`.
- Handlers convert request DTOs to input DTOs before calling usecases.

## Persistence

- Semantic enums use explicit `to_snake()` and `from_snake()` mappings.
- Repositories do not rely on `Display`, `to_string()`, or serde for database enum values.
- SQL bootstrap changes match repository expectations.

## Domain

- New concepts are explicit hospitality concepts, not generic framework abstractions.
- Static entities are not overloaded with temporal/current-state authority.
- Room state, occupancy, and housekeeping remain separate from static `Room` identity.

## Tests

- Normal cases are covered.
- Abnormal cases are covered.
- Invalid transitions are covered.
- Persistence round trips are covered for new semantic enums.
- Projection changes preserve deterministic convergence and rebuild/refresh equivalence.
