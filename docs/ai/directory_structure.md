# PMS-RS Directory Structure

The project intentionally uses explicit workflow-oriented structure.

Avoid introducing:

* generic shared layers
* common modules
* framework-centric abstractions
* utility dumping grounds

Preferred structure:

src
├── api
├── db
├── domain
├── error
├── lib.rs
├── main.rs
├── projection
├── repository
└── usecase

Guidelines:

* domain/ contains operational entities and domain rules
* usecases/ owns workflow orchestration and transaction boundaries
* repositories/ contains persistence adapters only
* projections/ contains rebuildable read models and materializers
* events/ contains behavioral/timeline event definitions
* api/ contains thin transport handlers only
* db/ contains schema and database setup
* tests/ contains integration-oriented workflow validation

Projection structure should remain explicit.

Preferred projection structure:

src/projections/
guest_summary/
model.rs
repository.rs
materializer.rs
refresh.rs
rebuild.rs

Avoid:

* generic projection registries
* dynamic projection dispatch
* hidden framework-style projection orchestration

Do not create new top-level architectural layers unless explicitly approved.
