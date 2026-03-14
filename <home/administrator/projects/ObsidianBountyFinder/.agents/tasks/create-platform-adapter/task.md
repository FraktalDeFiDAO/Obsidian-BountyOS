# Task: Create Platform Adapter

## Description
Scaffolds a new platform adapter for adding support for a new bounty platform.

## Steps
1. Create adapter module structure
2. Implement BountyAdapter trait
3. Add configuration handling
4. Create mock fixtures
5. Write unit tests
6. Add to workspace

## Template Structure
```
src/adapters/
└── new_platform/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   └── client.rs
    └── tests/
        ├── fixtures/
        └── integration_test.rs
```

## Implementation Checklist
- [ ] Platform enum variant
- [ ] Adapter struct with config
- [ ] HTTP client setup
- [ ] fetch_all() implementation
- [ ] fetch_updates() implementation
- [ ] Error handling
- [ ] Mock data
- [ ] Tests (>= 80%)

## Commands
```bash
# After creation
cargo test -p adapter-new-platform

# Lint
cargo clippy -p adapter-new-platform
```

## Integration
1. Add to `Cargo.toml` workspace
2. Add platform to enum
3. Register in sync manager
4. Add configuration to .env.example
5. Update documentation

## Example Platforms
- Immunefi
- HackenProof
- OpenZeppelin Security
- Trustless
- Layer3
