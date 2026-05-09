# PMS-RS AI Agent Policy

## General Principle

AI agents assist implementation but do not own architectural authority.

Human review is required for:

* architectural changes
* schema redesign
* aggregate boundary changes
* transaction model changes
* projection lifecycle redesign

---

# Allowed Actions

AI agents MAY:

* read repository files
* propose directory structures
* implement isolated workflow changes
* add integration tests
* add projection materializers
* add rebuild flows
* add documentation
* run formatting for touched files only
* run cargo test
* run targeted test commands
* run cargo check
* run cargo clippy
* inspect git diff/status

---

# Restricted Actions

AI agents MUST ask before:

* modifying Cargo.toml dependencies
* introducing new crates
* changing database schema significantly
* renaming top-level modules
* changing transaction boundaries
* changing projection ownership
* introducing framework abstractions
* introducing macros
* introducing async runtime changes
* changing API contracts
* performing large refactors
* editing unrelated files

---

# Forbidden Actions

AI agents MUST NOT:

* push directly to main
* bypass pull request workflow
* create repository-owned transactions
* introduce projection authority
* auto-refactor unrelated files
* mass-format untouched files
* introduce generic utility layers
* introduce premature abstractions
* delete large code sections without confirmation
* rewrite architecture without approval
* commit automatically
* push automatically
* resolve merge conflicts automatically

---

# Command Policy

Allowed commands:

* cargo test
* cargo check
* cargo fmt (touched files only)
* cargo clippy
* git diff
* git status

Ask before running:

* cargo fix
* cargo update
* database migrations
* destructive filesystem operations
* module renames
* dependency updates

Forbidden commands:

* git push main
* git reset --hard
* rm -rf
* mass rename operations
* automatic dependency upgrades

---

# Implementation Philosophy

Prefer:

* explicit workflows
* readable orchestration
* integration-first validation
* rebuildable projections
* transaction clarity
* explicit naming
* workflow-oriented code structure

Avoid:

* hidden magic
* framework-centric design
* excessive traits/generics
* speculative abstractions
* service locator patterns
* generic repository frameworks
* event bus overengineering
* unnecessary async abstraction

---

# Workflow Expectations

Before implementation:

1. Explain architectural impact
2. Explain transaction boundary ownership
3. Explain projection implications
4. Explain rebuildability considerations
5. Explain integration test strategy

Do not begin implementation until the architectural approach is clear.

---

# Testing Expectations

Prefer:

* integration tests
* transactional workflow validation
* projection rebuild validation
* API-level behavioral verification

Avoid:

* excessive mocking
* isolated repository-only testing
* framework-driven test abstractions

---

# Git Workflow Expectations

The repository uses:

feature branch
↓
develop
↓
main

Direct pushes to main are prohibited.

Pull requests should represent:

* one cohesive workflow change
* one architectural change
* one behavioral unit

Avoid unrelated mixed changes.

# Review Policy

Before completion:

1. generate review handoff document
2. save it to docs/ai/review/runtime/latest_review.md
3. generate the exact CLI review command
4. wait for user approval before review execution

sample command

```
codex review `
  --model gpt-5-codex `
  --prompt docs/ai/review/architecture_review_prompt.md `
  --context docs/ai/review/runtime/latest_review.md `
  --read-only
```

Before generating the review handoff:

Perform self-review for:

* transaction ownership violations
* projection authority leakage
* handler business logic
* rebuildability risks
* unnecessary abstraction

List all concerns explicitly.