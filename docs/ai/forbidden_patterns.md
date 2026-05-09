# PMS-RS Forbidden Patterns

The following patterns are intentionally prohibited unless explicitly redesigned and approved.

---

# Transaction Violations

Do NOT:

* open transactions inside repositories
* commit transactions inside repositories
* rollback transactions inside repositories
* allow projections to own transaction boundaries
* create nested transaction ownership implicitly

Transaction boundaries belong to:

* usecases
* orchestration layers

Repositories and projections consume existing transactions only.

---

# Projection Authority Violations

Do NOT:

* use projections as operational source-of-truth
* mutate operational state through projections
* reconstruct authoritative operational state from projections
* allow projections to block operational correction
* couple operational workflows directly to projection persistence

Projections are:

* rebuildable
* disposable
* non-authoritative
* read-oriented

Operational truth remains authoritative.

---

# Architectural Anti-Patterns

Do NOT introduce:

* generic service layers
* utility dumping grounds
* framework-centric abstractions
* hidden orchestration
* service locator patterns
* overengineered event buses
* speculative CQRS infrastructure
* premature generic repository frameworks
* unnecessary traits/generics
* ORM-style active record patterns

Prefer explicit workflow-oriented implementations.

---

# Handler Violations

Do NOT place business logic inside:

* API handlers
* controllers
* transport layers

Handlers should remain thin transport adapters only.

Workflow orchestration belongs in usecases.

---

# Projection Safety Violations

Do NOT:

* mutate projections without rebuildability
* bypass timeline/event recording
* tightly couple projections to operational schema evolution
* assume projections are always synchronized
* treat eventual consistency as transactional authority

Projection consistency must never override operational correctness.

---

# Testing Anti-Patterns

Avoid:

* excessive mocking
* repository-only testing
* framework-driven test abstractions
* implementation-detail-oriented tests

Prefer:

* integration tests
* workflow validation
* transactional consistency verification
* rebuild validation

---

# Git / Workflow Violations

Do NOT:

* push directly to main
* bypass pull request workflow
* auto-resolve merge conflicts
* auto-refactor unrelated files
* mass-format untouched files

Commits should represent:

* one workflow change
* one architectural change
* one cohesive behavioral unit

---

# File / Module Anti-Patterns

Avoid creating:

* utils.rs
* helper.rs
* common.rs
* shared.rs
* manager.rs

Prefer explicit workflow-oriented naming.