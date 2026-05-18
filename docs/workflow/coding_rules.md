# Coding Rules

## Error Handling

Use `AppError` for application errors and `ApiError` for transport errors.

Current application error categories:

- `Validation`
- `Domain`
- `NotFound`
- `Conflict`
- `Infrastructure`

Handlers map `AppError` to HTTP responses through `map_app_error`.

## Domain

`domain/entity` represents operational truth and structural identity.

Entities may contain identifiers, immutable operational facts, timestamps, and semantic enums. They must not contain projection responsibility, derived balance authority, mutable settlement truth, or projection-owned current-state authority.

`domain/semantic` represents operational interpretation, relation semantics, and settlement semantics. Semantic objects may express workflows such as settlement state, billing responsibility, reservation relation semantics, or room daily state. They must not own persistence or transactions.

Prefer explicit hospitality concepts over generic abstractions.

## DTOs

Request DTOs are transport boundaries.

Request DTOs may contain strings, primitive JSON values, and semantic enums. They must not contain parsed UUIDs, parsed dates, parsed datetimes, `Decimal`, or workflow orchestration semantics.

Input DTOs are transport-independent workflow inputs.

Input DTOs may contain parsed UUIDs, dates, datetimes, `Decimal`, and semantic enums. They must not contain serde, HTTP, JSON, or transport semantics.

Response DTOs are semantic-preserving transport outputs.

Response DTOs should keep semantic enums as enums. Do not flatten semantic status fields into strings manually.

## Semantic Enums

All semantic enums exposed through serde must use:

```rust
#[serde(rename_all = "snake_case")]
```

All semantic enums persisted to the database must implement:

```rust
to_snake()
from_snake()
```

Repositories must use these explicit mapping methods. Do not rely on `Display`, `to_string()`, or implicit serde conversion for database values.

## Handlers

Handlers are transport normalization boundaries.

Handlers parse path/query/body values, convert request DTOs to input DTOs, call `usecase::execute(&state.db, input)`, convert domain results to response DTOs, and map errors.

Handlers must not own transactions, call repositories directly, contain workflow orchestration, or mutate projection state.

Preferred shape:

```rust
pub async fn some_handler(
    State(state): State<AppState>,
    Json(req): Json<SomeRequest>,
) -> Result<(StatusCode, Json<SomeResponse>), ApiError> {
    let input = SomeInput { /* parsed fields */ };
    let result = some_usecase::execute(&state.db, input).await.map_err(map_app_error)?;
    Ok((StatusCode::OK, Json(SomeResponse::from(result))))
}
```

Path extractors are allowed when the route identity is naturally path-owned.

## Usecases

Usecases are transactional workflow orchestrators.

Command usecases own write transactions. They load authoritative state, validate workflow semantics, persist operational changes, record append-only history where appropriate, trigger topology/orchestrator-owned projection propagation where appropriate, and commit or roll back.

Detail/search usecases may use read transactions and roll back after read-only work.

Usecase files live under:

- `command` for mutations and workflow transitions
- `detail` for single-resource reads
- `search` for lists and query-style reads

Usecase file names are the usecase names. The callable function is `execute`.

## Repositories

Repositories are persistence boundaries only.

Repositories may save, update, find, map rows, and explicitly map semantic enums. They must not own transactions, orchestrate workflows, contain business branching, call projections, or call usecases.

## Projections

Projection components are local-only. They may materialize, refresh, persist, and rebuild local projection state.

Projection propagation belongs to topology/orchestrator layers.

Projection state must never become operational authority or settlement authority.

## Calculations

Analytical calculations normally belong to projections, aggregates, or derived models.

Operational/accounting calculations required for workflow correctness may exist under `usecase/*/calculation`, but they must derive from authoritative operational/accounting records and must not depend on projection state.

## Testing

Add normal and abnormal cases when implementing workflows.

Prefer integration tests for operational behavior. Tests should assert authority boundaries, invalid transitions, persistence mapping, and projection rebuild/refresh invariants when relevant.
