# PMS-RS Git Workflow Policy

## Branch Strategy

The repository uses a staged integration workflow:

feature branch
↓
develop
↓
main

Direct pushes to `main` are prohibited.

---

## Main Branch Policy

`main` represents:

* stable architectural state
* validated operational workflows
* integration-confirmed behavior

All changes must be merged through Pull Requests.

---

## Recommended Flow

feature/*
↓
Pull Request
↓
develop
↓
stabilization / validation
↓
Pull Request
↓
main

---

## Pull Request Expectations

Pull Requests should:

* represent a cohesive architectural change
* preserve transactional correctness
* preserve projection consistency
* include relevant integration tests
* avoid unrelated mixed changes

---

## Architectural Priority

Architectural consistency is prioritized over development speed.

Temporary shortcuts that violate:

* transaction ownership
* projection rebuildability
* operational authority boundaries

are intentionally discouraged.
