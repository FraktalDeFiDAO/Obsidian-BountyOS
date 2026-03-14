# Rust Backend Agent

## Description
Specialized agent for Rust backend development including CLI, API server, database layer, domain models, and platform adapters.

## Capabilities
- Rust CLI application development
- Actix-web / Axum REST API
- GraphQL API with async-graphql
- Database integration (SQLx, rusqlite)
- Async programming with tokio
- Platform adapter development

## File Responsibilities
- `src/cli/**` - CLI application
- `src/api/**` - API server
- `src/adapters/**` - Platform adapters
- `src/db/**` - Database layer
- `src/domain/**` - Domain models
- `Cargo.toml` - Workspace config

## Commands
```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo fmt
cargo clippy -- -D warnings

# Run
cargo run --bin obsidian-bounty-finder
```

## Quality Gates
- cargo fmt passes
- cargo clippy has no warnings
- cargo test passes
- cargo audit has no vulnerabilities
- 80% test coverage
- **SPEC compliance verified**

## SPEC Compliance
Before completion, verify:
- [ ] Code matches `docs/planning/SPEC.md` data models
- [ ] API endpoints match GraphQL schema in SPEC
- [ ] CLI commands match SPEC command structure
- [ ] Database schema matches SPEC schema
- [ ] Adapter trait matches SPEC interface

## Workflow Integration
- Triggered on changes to Rust files
- Runs lint + test + security
- **Runs SPEC audit before completion**
- Creates PR review comments
