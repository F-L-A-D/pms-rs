# Domain Expansion Guide

When adding hospitality domain behavior, start from authority.

Ask:

- What is the authoritative operational record?
- Is the concept current state, append-only history, semantic interpretation, or projection?
- Who owns the transaction?
- What invalid transitions must be rejected?
- What values must be persisted as explicit snake_case enums?

## Preferred Flow

1. Add or refine domain entity/semantic types.
2. Add bootstrap schema only when persistence is needed.
3. Add repository methods that consume an existing transaction.
4. Add input/request/response DTOs at the appropriate boundaries.
5. Add usecases with `execute`.
6. Add API handlers only after the usecase boundary is clear.
7. Add normal and abnormal tests.

## Room And Housekeeping

`Room` is static room identity and structural information.

Date-scoped room state belongs to `RoomDailyState`.

Housekeeping operations should mutate `RoomDailyState` for a service date. They should not fabricate state in `RoomResponse`.

Future room expansion should prefer explicit concepts such as:

- room daily occupancy
- housekeeping task lifecycle
- out-of-order periods
- maintenance holds
- stay-to-room occupancy synchronization

## Billing And Settlement

Keep these boundaries distinct:

- Folio: operational billing container
- FolioEntry: billing event/history item
- BillingAccount: settlement responsibility owner
- Invoice: immutable settlement snapshot
- Receivable: outstanding liability
- Payment: settlement mutation

Do not use guest identity, reservation participation, or projection-derived values as settlement authority.
