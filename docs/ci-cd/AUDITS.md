# Audit Pipeline Specifications

## ObsidianBountyFinder

**Version:** 1.0  
**Last Updated:** 2026-03-14

---

## 1. Audit Overview

Comprehensive security and compliance audits across all technology stacks.

### Audit Matrix

| Flavor | Tool | Type | Frequency | Auto-Fix |
|--------|------|------|-----------|----------|
| **Rust** | cargo-audit | Security | Daily + PR | ✅ |
| **Rust** | cargo-deny | License | Daily + PR | ❌ |
| **Rust** | clippy | Quality | PR | ❌ |
| **JS/TS** | npm audit | Security | Daily + PR | ✅ |
| **JS/TS** | snyk | Security | Daily + PR | ✅ |
| **JS/TS** | eslint | Quality | PR | ❌ |
| **Docker** | trivy | Security | Daily + PR | ❌ |
| **Docker** | hadolint | Quality | PR | ❌ |
| **Secrets** | trufflehog | Security | Daily + PR | ❌ |
| **IaC** | checkov | Compliance | PR | ❌ |

---

## 2. Security Workflow (.github/workflows/security.yml)

### Triggers

```yaml
on:
  pull_request:
  push:
    branches: [main, develop]
  schedule:
    - cron: '0 0 * * *'  # Daily at midnight
  workflow_dispatch:
```

### Jobs

#### Rust Audits
```yaml
rust-audit:
  name: Rust Security Audit
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    - name: Run cargo-audit
      run: cargo audit --json | tee audit.json
    - name: Upload results
      uses: actions/upload-artifact@v4
      with:
        name: rust-audit-results
        path: audit.json
```

```yaml
rust-license:
  name: Rust License Compliance
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Run cargo-deny
      run: cargo deny check
```

#### JavaScript Audits
```yaml
npm-audit:
  name: NPM Security Audit
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - run: npm ci
    - run: npm audit --audit-level=moderate
    - name: Run Snyk
      run: npx snyk test
```

#### Docker Audits
```yaml
docker-security:
  name: Docker Security Scan
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Run Trivy
      run: trivy fs --severity HIGH,CRITICAL .
```

```yaml
hadolint:
  name: Hadolint
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Lint Dockerfiles
      run: hadolint Dockerfile*
```

#### Secrets Scanning
```yaml
secrets-scan:
  name: Secrets Detection
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: TruffleHog
      run: trufflehog filesystem . --json
```

---

## 3. Severity Levels

| Level | Description | Action |
|-------|-------------|--------|
| **Critical** | RCE, data breach | Block + Notify |
| **High** | Privilege escalation | Block + Notify |
| **Medium** | Information disclosure | Warn + Notify |
| **Low** | Best practice | Warn |

---

## 4. Auto-remediation

### Enabled Fixes

| Tool | Command |
|------|---------|
| cargo-audit | `cargo audit` (shows advisory, manual fix) |
| npm audit | `npm audit fix` |
| Snyk | `snyk wizard` |

### Process
1. Audit finds vulnerability
2. Check if auto-fixable
3. Apply fix if safe
4. Create PR with changes
5. Notify team

---

## 5. Notifications

### Channels
- Discord (webhook)
- Telegram (bot)
- Email (SMTP)
- PR comments

### Notification Workflow

```yaml
audit-notify:
  name: Audit Notification
  needs: [rust-audit, npm-audit, docker-security, secrets-scan]
  if: always()
  runs-on: ubuntu-latest
  steps:
    - name: Aggregate results
      run: |
        # Collect all audit results
        # Send to notification channels
```

---

## 6. .agents Integration

### Audit Skills

Each audit type has a corresponding skill in `.agents/skills/`:

```
.agents/skills/
├── rust_security_audit/
│   ├── SKILL.md
│   └── prompts/audit_report.md
├── js_security_audit/
├── docker_security_audit/
├── secrets_scanning/
└── license_compliance/
```

### Skill Configuration

```yaml
# .agents/skills/rust_security_audit/SKILL.md
name: Rust Security Audit
tools:
  - cargo-audit
  - cargo-deny
triggers:
  - schedule: "0 0 * * *"
  - pr_opened
  - pr_synchronized
actions:
  - run_audit
  - generate_report
  - notify_if_critical
```

---

## 7. Running Audits Locally

```bash
# All audits
make audit

# Specific audit
make audit-rust
make audit-js
make audit-docker
make audit-secrets

# With act
act --workflow security.yml
```

---

## 8. Reporting

### Daily Reports
- Aggregated security status
- New vulnerabilities found
- Fixes applied

### PR Reports
- Audit results in PR check
- Links to detailed reports
- Fix suggestions

### Storage
- Artifacts retained for 90 days
- Detailed logs in GitHub
- External SIEM integration (optional)
