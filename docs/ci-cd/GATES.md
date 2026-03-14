# Development Gates

## ObsidianBountyFinder

**Version:** 1.0  
**Last Updated:** 2026-03-14

---

## 1. Gate Overview

Development gates ensure code quality, security, and consistency before merging to protected branches.

---

## 2. Pre-commit Gates

### Local Hooks (.pre-commit-config.yaml)

```yaml
repos:
  # Rust
  - repo: https://github.com/rust-lang/rustfmt
    hooks: [rustfmt]
    
  - repo: https://github.com/rust-lang/rust-clippy
    hooks: [clippy]
    
  # JavaScript/TypeScript
  - repo: https://github.com/prettier/prettier
    hooks: [prettier]
    
  - repo: https://github.com/eslint/eslint
    hooks: [eslint]
    
  # General
  - repo: https://github.com/pre-commit/pre-commit-hooks
    hooks:
      - trailing-whitespace
      - end-of-file-fixer
      - check-yaml
      - check-json
      
  # Secrets
  - repo: https://github.com/trufflesecurity/trufflehog
    hooks: [trufflehog]
```

### Running Pre-commit

```bash
# Install hooks
make setup

# Run manually
make pre-commit

# Skip specific hooks
SKIP=trufflehog make pre-commit
```

---

## 3. PR Gates

### Required Checks

| Check | Tool | Pass Condition |
|-------|------|----------------|
| Format | cargo fmt | No changes |
| Lint | cargo clippy | No warnings |
| Test | cargo test | All pass |
| Coverage | tarpaulin | ≥80% |
| Security | cargo audit | No critical/high |
| Secrets | trufflehog | No findings |

### PR Requirements

- [ ] All CI checks pass
- [ ] 80% test coverage
- [ ] No critical/high vulnerabilities
- [ ] At least 1 approval (2 for security changes)
- [ ] No unresolved comments
- [ ] Branch up-to-date with target

---

## 4. Branch Protection

### main Branch Rules

```yaml
# GitHub Branch Protection
required_status_checks:
  - ci  # All CI checks must pass
  
restrictions:
  # Require PR
  required_reviewers: 1
  
# Require signed commits (optional)
required_commit_signatures: true
```

---

## 5. Quality Thresholds

| Metric | Threshold | Action |
|--------|-----------|--------|
| Test Coverage | ≥80% | Block merge |
| Clippy Warnings | 0 | Block merge |
| Security Vulns | 0 Critical/High | Block merge |
| Secrets Found | 0 | Block merge |
| Lint Errors | 0 | Block merge |

---

## 6. Fast-fail Strategy

Jobs run in parallel where possible. On failure:

1. **Lint failures** → Fail fast, show errors immediately
2. **Test failures** → Run after lint, show failing tests
3. **Security failures** → Run last, show vulnerability report
4. **Coverage failures** → Informational, not blocking

---

## 7. Enforcement

### GitHub Actions

```yaml
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run formatting
        run: cargo fmt -- --check
        
      - name: Run clippy
        run: cargo clippy -- -D warnings
        
      - name: Run tests
        run: cargo test
        
      - name: Check coverage
        run: |
          cargo tarpaulin --out Xml --coveralls-xml
          # Upload to codecov
```

### Pre-commit Hook Installation

```bash
# Install pre-commit
pip install pre-commit

# Initialize
pre-commit install

# Update hooks
pre-commit autoupdate
```
