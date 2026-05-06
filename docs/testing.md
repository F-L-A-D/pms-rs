# PMS-RS Testing Strategy

## Goal

Ensure operational consistency and reproducible behavior.

The system handles:
- reservation lifecycle
- room state transitions
- billing events
- transactional consistency

Testing is treated as part of domain validation.

---

# Test Principles

## Success and Failure Cases

Every important usecase should include:
- success pattern
- failure pattern

Examples:
- successful check-in
- failed check-in due to invalid room state

---

## State Transition Validation

Tests should verify:
- before state
- operation
- after state

Examples:
- Vacant -> Occupied
- Dirty -> Cleaning

---

## Persistence Validation

State-changing operations should verify:
- DB persistence
- repository consistency

---

## Transaction Validation

Transactional operations should verify:
- commit behavior
- rollback behavior
- inventory consistency

---

## Ledger Validation

Billing tests should verify:
- entries are append-only
- balance is derived from entries
- corrections are additive instead of destructive

---

# Test Categories

## Domain Tests

Focus:
- entity rules
- validation
- state transitions

Examples:
- Folio close twice should fail
- Invalid reservation dates should fail

---

## Integration Tests

Focus:
- usecase execution
- repository interaction
- DB consistency

Examples:
- reservation create
- room assignment
- billing entry posting

---

## Transaction Tests

Focus:
- rollback safety
- consistency guarantees

Examples:
- inventory rollback
- failed assignment rollback

---

# Naming Convention

Use:
- should_xxx
- should_fail_xxx

Examples:
- should_create_reservation
- should_fail_assign_wrong_room_class

---

# API Testing (Planned)

Future API tests should validate:
- HTTP status codes
- JSON responses
- request validation
- serialization consistency

---

# Future Expansion

Future milestones may introduce:
- concurrency tests
- load tests
- event replay consistency
- API contract tests