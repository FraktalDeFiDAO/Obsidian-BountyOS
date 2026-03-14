# CI/CD Pipeline Documentation

## ObsidianBountyFinder

**Version:** 1.0  
**Last Updated:** 2026-03-14

---

## 1. Pipeline Overview

### Workflow Files

| File | Trigger | Purpose |
|------|---------|---------|
| `ci.yml` | PR, push | Lint, test, build |
| `cd.yml` | Tag (v*) | Deploy to production |
| `security.yml` | Daily, PR, push | Security audits |
| `audit-notify.yml` | Dispatch | Notification dispatcher |

### Pipeline Stages

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│  Lint   │ →  │  Test   │ →  │  Build  │ →  │ Security│
└─────────┘    └─────────┘    └─────────┘    └─────────┘
     │              │              │              │
     ▼              ▼              ▼              ▼
  rustfmt       cargo test    cargo build    cargo audit
  clippy        vitest        npm build      npm audit
  eslint        coverage      docker build   trivy
  prettier                                   trufflehog
```

---

## 2. CI Pipeline (ci.yml)

### Jobs

#### Lint
```yaml
lint-rust:
  - run: cargo fmt -- --check
  - run: cargo clippy -- -D warnings
  
lint-js:
  - run: npm run lint
  - run: npm run typecheck
```

#### Test
```yaml
test-rust:
  - run: cargo test --workspace --all-features
  - run: cargo test --workspace --doc
  
test-js:
  - run: npm run test -- --coverage
```

#### Build
```yaml
build-rust:
  - run: cargo build --release
  
build-docker:
  - run: docker build -t obsidian-bounty-finder:${{ github.sha }} .
```

---

## 3. CD Pipeline (cd.yml)

### Triggers
- Tag matching `v*` (e.g., v1.0.0)

### Stages
1. Build Docker image
2. Push to registry
3. Deploy to production

---

## 4. Security Pipeline (security.yml)

### Scheduled Runs
- Daily at 00:00 UTC

### Audit Jobs

| Job | Tool | Frequency |
|-----|------|-----------|
| rust-audit | cargo-audit | PR + Daily |
| rust-deny | cargo-deny | PR + Daily |
| npm-audit | npm audit | PR + Daily |
| snyk | snyk | PR + Daily |
| trivy | trivy | PR + Daily |
| hadolint | hadolint | PR |
| trufflehog | trufflehog | PR + Daily |

---

## 5. Local Execution with act

### Installation
```bash
# macOS
brew install act

# Linux
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

### Configuration (.actrc)
```bash
-P ubuntu-latest=ghcr.io/catthehacker/ubuntu:runner-latest
--workflows .github/workflows/ci.yml
--env NODE_ENV=test
```

### Running
```bash
# Run PR workflow
act pull_request

# Run all workflows
act

# Run specific job
act -j lint-rust
```

---

## 6. Development Workflow

### Pre-commit (Local)
```bash
# Install hooks
make setup

# Run manually
make pre-commit
```

### PR Process
1. Create feature branch
2. Make changes
3. Run `make pre-commit`
4. Push and open PR
5. CI runs all checks
6. Review and merge

### Release Process
1. Update version in `Cargo.toml`
2. Create tag: `git tag v1.0.0`
3. Push tag: `git push origin v1.0.0`
4. CD pipeline deploys

---

## 7. Environment Configuration

### CI Variables
| Variable | Description |
|----------|-------------|
| `CODECOV_TOKEN` | Code coverage upload |
| `DOCKER_USERNAME` | Container registry |
| `DOCKER_PASSWORD` | Container registry |
| `SNYK_TOKEN` | Snyk vulnerability scanning |

### Secrets (Required)
- `GITHUB_TOKEN` (auto-provided)
- `DOCKER_PASSWORD`
- `CODECOV_TOKEN`
