# PMS-RS Conventions

# Naming Conventions

## General Principles

Names should prioritize:

* explicit intent
* architectural role clarity
* workflow readability

Avoid:

* abbreviations
* overloaded terminology
* framework-centric naming

---

# Layer Naming

## Usecase

Usecases represent operational workflows.

Pattern:

* create_reservation
* check_in
* post_room_charge
* refresh_guest_summary_projection

Avoid:

* service
* manager
* util

---

## Repository

Repositories are persistence adapters only.

Pattern:

* SqliteReservationRepository
* SqliteGuestRepository

Repository methods should describe persistence behavior:

* save
* update
* find_by_id
* find_all
* delete_all

Avoid:

* business terminology
* orchestration logic

---

## Projection

Projection naming must distinguish:

* projection model
* materialization
* rebuild
* refresh

Pattern:

* GuestSummaryProjection
* materialize_guest_summary
* rebuild_guest_summary_projection
* refresh_guest_summary_projection

---

## Timeline / Event

Behavioral history must use event-oriented terminology.

Pattern:

* TimelineEventType
* record_event
* guest_timeline_event

Avoid:

* audit
* log
* history_entry

unless semantically different.

---

# Test Naming

Tests should describe observable workflow behavior.

Pattern:

* should_create_reservation
* should_refresh_projection_after_reservation_created
* should_rebuild_guest_summary_projection

Avoid:

* vague technical descriptions
* implementation-specific naming

---

# File Naming

Use snake_case consistently.

Avoid:

* mixed conventions
* framework-generated naming styles
* generic filenames like utils.rs, helper.rs, manager.rs, common.rs

---

# Commit Message Conventions

## Principles

Commit messages should describe:

* architectural intent
* workflow impact
* behavioral changes

Avoid:

* vague summaries
* implementation-only wording

---

## Format

<type>: <summary>

Examples:

* feat: implement guest summary projection layer
* fix: unify transaction ownership policy
* refactor: separate projection refresh flow
* test: add projection rebuild integration tests
* docs: update projection architecture documentation

---

## Recommended Types

* feat
* fix
* refactor
* test
* docs
* chore

Avoid excessive granularity.

---

## Commit Scope Philosophy

A commit should represent:

* one architectural change
* one workflow change
* one cohesive behavioral unit

Avoid:

* unrelated mixed commits
* formatting-only noise mixed with logic changes
* large ambiguous commits
