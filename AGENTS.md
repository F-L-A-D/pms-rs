# PMS-RS AI Agent Instructions

PMS-RS is not a generic CRUD application.

The architecture prioritizes:

- operational correctness
- transactional reproducibility
- projection rebuildability
- explicit authority boundaries
- deterministic projection convergence
- practical hospitality domain semantics

Operational correctness has priority over projection consistency.

Projections are derived, rebuildable, disposable, and non-authoritative. They must never own transactions, become operational authority, become settlement authority, directly orchestrate downstream propagation, or block operational corrections because projection state is stale.

Repositories never own transactions. Transaction ownership belongs only to usecases and orchestration layers.

Projection propagation is topology-managed and orchestrator-owned. Projection services remain local-only.

Do not introduce generic framework abstractions such as `common.rs`, `utils.rs`, `helper.rs`, `BaseService`, `GenericRepository`, manager-style abstractions, service locators, implicit propagation, hidden orchestration, or speculative framework layers.

Before large modifications, review:

- `docs/workflow/README.md`
- `docs/workflow/current_state.md`
- `docs/workflow/architecture_principles.md`
- `docs/workflow/coding_rules.md`
- `docs/workflow/review_checklist.md`

For historical reasoning, use `docs/core`, `docs/architecture`, and `docs/roadmap` only when needed.
