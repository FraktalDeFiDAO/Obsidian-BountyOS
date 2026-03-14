# Rust Backend Skill

## Description
Specialized skill for Rust backend development including CLI, API, database, and domain logic.

## Capabilities
- Rust CLI application development (clap, ratatui)
- Actix-web / Axum API development
- Database integration (SQLx, rusqlite, diesel)
- Async programming (tokio)
- GraphQL APIs (async-graphql)

## Tools
- cargo
- rustc
- rustfmt
- clippy
- cargo-audit
- cargo-deny

## File Patterns
- `src/cli/**`
- `src/api/**`
- `src/db/**`
- `src/domain/**`
- `Cargo.toml`

## Commands
```bash
# Build
cargo build --release

# Test
cargo test --workspace

# Lint
cargo fmt
cargo clippy -- -D warnings

# Audit
cargo audit
cargo deny check
```

## Triggers
- Changes to Rust source files
- Cargo.toml updates
- CI/CD pipeline runs

## Workflows
- pr_review
- code_quality_check
- security_audit
