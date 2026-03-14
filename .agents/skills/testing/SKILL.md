# Testing Skill

## Description
Comprehensive testing strategy including unit, integration, and E2E tests.

## Capabilities
- Unit testing (Rust + JavaScript)
- Integration testing
- E2E testing
- Test coverage analysis
- Property-based testing
- Mutation testing

## Rust Testing Tools
- cargo test
- cargo-tarpaulin (coverage)
- proptest (property-based)
- mockall (mocking)
- httpmock (HTTP mocking)

## JavaScript Testing Tools
- vitest
- @testing-library/vue
- @testing-library/user-event
- msw (mock service worker)
- playwright (E2E)

## File Patterns
- `**/*_test.rs`
- `**/tests/**/*.rs`
- `src/**/*.test.ts`
- `src/**/*.spec.ts`
- `e2e/**/*`

## Commands

### Rust
```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Xml

# Property-based tests
cargo test --all-features

# Doc tests
cargo test --doc
```

### JavaScript
```bash
# Run tests
npm run test

# Watch mode
npm run test:watch

# Coverage
npm run test:coverage

# E2E tests
npx playwright test
```

## Coverage Targets
| Level | Target |
|-------|--------|
| Unit | 85% |
| Integration | 90% |
| Overall | 80% |

## Test Structure
```
src/
├── module/
│   ├── mod.rs
│   └── tests/
│       ├── mod.rs
│       ├── integration_test.rs
│       └── fixtures/
└── ...

frontend/
├── src/
│   └── __tests__/
│       ├── unit/
│       └── integration/
└── e2e/
```

## CI Integration
- Run on every PR
- Coverage gates (80% minimum)
- Parallel execution
- Artifact retention

## Triggers
- Pull requests
- Push to main/develop
- Scheduled nightly runs
