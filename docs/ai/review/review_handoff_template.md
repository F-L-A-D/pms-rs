# docs/ai/review/review_handoff_template.md

# Review Handoff

## Summary

Short description of the implementation.

Example:

* implemented guest search projection
* added rebuild flow
* added integration tests

---

## Scope

Describe the intended implementation scope.

Example:

* projection layer only
* no schema redesign
* no transaction redesign

---

## Modified Files

List all modified files.

Example:

* src/usecases/check_in.rs
* src/projections/guest_summary/materializer.rs
* src/tests/check_in_projection_test.rs

---

## Workflow Changes

Describe:

* operational workflow changes
* orchestration changes
* state transition changes

Questions to answer:

* what operational flow changed?
* what triggers were added?
* what orchestration behavior changed?

---

## Transaction Boundary

Describe:

* transaction owner
* transaction participants
* transaction expectations

Questions to answer:

* which usecase owns the transaction?
* which repositories participate?
* which projections consume the transaction?
* was transaction ownership changed?

---

## Projection Impacts

Describe:

* affected projections
* refresh timing
* rebuildability implications
* synchronization expectations

Questions to answer:

* which projections changed?
* are projections still rebuildable?
* was projection authority introduced?
* are projections eventually consistent or transactionally synchronized?

---

## API Impacts

Describe:

* endpoint changes
* request/response changes
* handler behavior changes

Questions to answer:

* were handlers kept thin?
* was business logic added to handlers?
* were API contracts changed?

---

## Event / Timeline Impacts

Describe:

* new events
* modified event propagation
* timeline consistency implications

Questions to answer:

* were new timeline events added?
* was event ordering affected?
* does rebuildability remain preserved?

---

## Integration Tests

List:

* added tests
* modified tests
* workflow validations

Questions to answer:

* what workflows are validated?
* are projection rebuilds tested?
* are transaction boundaries validated?

---

## Architectural Risks

Describe:

* known tradeoffs
* temporary compromises
* future concerns

Questions to answer:

* what architectural risks exist?
* what may require future redesign?
* what assumptions were introduced?

---

## Open Questions

Describe unresolved architectural concerns.

Example:

* should projection refresh remain synchronous?
* should guest metrics remain derived-on-read?
