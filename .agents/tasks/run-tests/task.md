# Task: Run Tests

## Description
Runs the full test suite including unit, integration, and E2E tests.

## Steps
1. Run Rust tests
2. Run JavaScript tests
3. Generate coverage reports
4. Check coverage thresholds
5. Upload coverage to codecov

## Tools
- cargo test
- vitest
- cargo-tarpaulin
- codecov

## Commands
```bash
# Full test suite
make test

# Rust only
cargo test --workspace

# Frontend only
cd frontend && npm run test

# With coverage
make test-coverage
```

## Coverage Thresholds
| Level | Target |
|-------|--------|
| Unit | 85% |
| Integration | 90% |
| Overall | 80% |

## Output
- Test results
- Coverage report
- Codecov upload

## Success Criteria
- All tests pass
- Coverage >= 80%
- No flaky tests

## Failure Handling
- Show failing tests
- Block merge if below threshold
- Generate coverage diff
