# Testing Strategy

## ObsidianBountyFinder

**Version:** 1.0  
**Last Updated:** 2026-03-14

---

## 1. Test Philosophy

We follow the **test pyramid** approach with emphasis on:
- **Fast feedback** - Unit tests run on every PR
- **Reliability** - Integration tests verify component interactions
- **Coverage** - E2E tests cover critical user flows

---

## 2. Test Pyramid

```
           ┌─────────────┐
           │     E2E     │  ← Playwright/Cypress (10%)
           ├─────────────┤
           │ Integration │  ← API tests, adapter tests (20%)
           ├─────────────┤
           │   Unit      │  ← cargo test, vitest (70%)
           └─────────────┘
```

### Coverage Targets

| Level | Target | Minimum |
|-------|--------|---------|
| Unit | 85% | 80% |
| Integration | 90% | 85% |
| E2E | Critical paths | All |

---

## 3. Testing Tools

### Rust (Backend)
| Tool | Purpose |
|------|---------|
| `cargo test` | Unit & integration tests |
| `cargo-tarpaulin` | Code coverage |
| `proptest` | Property-based testing |
| `mockall` | Mocking |
| `httpmock` | HTTP mocking |

### JavaScript/TypeScript (Frontend)
| Tool | Purpose |
|------|---------|
| `vitest` | Unit tests |
| `playwright` | E2E tests |
| `testing-library` | Component tests |
| `msw` | API mocking |

---

## 4. Test Structure

```
src/
├── cli/
│   └── tests/
├── api/
│   └── tests/
├── adapters/
│   ├── github/
│   │   ├── tests/
│   │   └── fixtures/
│   └── ...
└── ...

frontend/
├── src/
│   └── __tests__/
└── e2e/
```

---

## 5. Platform Adapter Testing

### Mock Servers
Each platform adapter includes mock data:

```
adapters/github/tests/fixtures/
├── issues_page1.json
├── issues_page2.json
└── issue_detail.json

adapters/gitcoin/tests/fixtures/
└── grants.json
```

### Testing Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_fetch_all_returns_bounties() {
        // Use mock HTTP server
        let mock = MockServer::start();
        mock.expect(
            Request::new("/issues")
                .with_header("Authorization", "Bearer token")
        )
        .respond_with(
            Response::new(200, include_str!("fixtures/issues.json"))
        );
        
        let adapter = GitHubAdapter::new_with_url(mock.url());
        let result = adapter.fetch_all().await;
        
        assert!(result.is_ok());
    }
}
```

---

## 6. Running Tests

### Local
```bash
# All tests
make test

# Rust only
cargo test --workspace

# Frontend only
cd frontend && npm run test

# With coverage
make test-coverage
```

### CI
```bash
# Run with act
act pull_request
```

---

## 7. Test Data Management

### Fixtures
- JSON files in `tests/fixtures/`
- Recorded API responses
- Scenario-based test cases

### Test Databases
- SQLite in-memory for unit tests
- Test containers for integration tests

---

## 8. Continuous Testing

### Pre-commit
- All unit tests must pass
- Coverage check (80% minimum)

### PR Checks
- Full test suite
- Coverage report
- Mutation testing (optional)

### Nightly
- Full E2E suite
- Performance benchmarks
- Security scans
