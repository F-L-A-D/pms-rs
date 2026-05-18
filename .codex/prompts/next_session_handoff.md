# PMS-RS Next Session Handoff

Think in English.
Respond in Japanese.

You are onboarding into PMS-RS after the `feature/hospitality-domain-expansion` branch work.

Before implementing anything, read:

- `AGENTS.md`
- `docs/workflow/README.md`
- `docs/workflow/current_state.md`
- `docs/workflow/architecture_principles.md`
- `docs/workflow/coding_rules.md`
- `docs/workflow/review_checklist.md`

Core constraints to preserve:

- PMS-RS is not a generic CRUD application.
- Operational correctness has priority over projection consistency.
- Operational truth is authoritative.
- Projections are derived, rebuildable, disposable, and non-authoritative.
- Repositories never own transactions.
- Transaction ownership belongs to usecases and orchestration layers.
- Projection propagation is topology-managed and orchestrator-owned.
- Projection services remain local-only.
- Do not introduce generic framework abstractions such as `common.rs`, `utils.rs`, generic repositories, manager-style abstractions, service locators, hidden orchestration, or speculative framework layers.

Current branch:

- `feature/hospitality-domain-expansion`
- PR target: `develop`

Recent work completed:

- Reorganized workflow docs and coding rules.
- Split `Room` static information from date-aware `RoomDailyState`.
- Added room daily state operational persistence and housekeeping lifecycle around the date-aware state.
- Added inventory aggregate projection by service date and room class.
- Added housekeeping daily workload aggregate projection.
- Added reservation booking enrichment:
  - `booking_channel`
  - `plan_code`
  - package revenue breakdowns
  - daily stay details
  - daily revenue allocations
- Added daily/monthly KPI aggregates:
  - room-class level
  - hotel-wide level
  - occupancy / ADR / RevPAR / revenue categories
- Added semantic change flow:
  - `OperationContext`
  - `OperationChangeEvent`
  - `ChangePattern`
  - `ConfidenceProfile`
  - `SemanticActivation`
- Connected reservation create / modify / cancel to operation change events in the same usecase transaction as the reservation write.
- Added `GET /operation-events/:id/semantic-signal` as a read-only signal endpoint.
- Added tests for:
  - semantic signal classification
  - refresh / rebuild parity
  - projection runtime topology order
  - missing projection rows returning `null` instead of blocking operational reads

Important recent commits:

- `7cc5ab4 feat: add inventory aggregate projection`
- `68599e3 feat: add housekeeping workload aggregate projection`
- `5108cba feat: add reservation booking and package breakdowns`
- `b979067 feat: add daily room class kpi aggregate`
- `d84c462 feat: add hotel and monthly kpi aggregates`
- `99baa99 feat: add reservation daily stay allocations`
- `c7859c5 feat: add semantic activation change flow`
- `5da63b3 feat: refine operation change semantics`
- `cdd6cf1 feat: expose operation semantic signal`
- `e4b9535 test: cover semantic signal projection flow`

Current state:

- `cargo check` passes.
- `cargo test --test api` passes.
- `cargo test --test projection` passes.
- `cargo test` passes.
- Working tree should be clean after this handoff commit.

Recommended next scope:

1. Review the PR for architectural consistency.
2. If continuing implementation, the next natural scope is reservation modification granularity:
   - allow modifying daily stay details explicitly
   - allow modifying package breakdowns / daily revenue allocations explicitly
   - emit structured `OperationChangeEvent.changed_fields_json`
   - keep business authority in operational usecases
   - keep `SemanticActivation` as derived signal only
3. Do not connect semantic activation as an orchestration authority.
4. Do not redesign projection runtime.
5. Do not introduce generic framework abstractions.

When reviewing or extending the work, watch especially for:

- projection accidentally becoming operational authority
- handler logic gaining business semantics
- repository methods owning transactions
- downstream propagation bypassing topology/orchestrator
- KPI/inventory aggregates treating decision-support data as truth
- semantic activation being used as command authority rather than signal
