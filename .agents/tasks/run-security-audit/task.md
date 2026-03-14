# Task: Run Full Security Audit

## Description
Runs comprehensive security audits across all technology stacks.

## Steps
1. Run Rust security audit (cargo-audit, cargo-deny)
2. Run JavaScript security audit (npm audit, snyk)
3. Run Docker security scan (trivy, hadolint)
4. Run secrets detection (trufflehog)
5. Generate aggregated report
6. Notify on critical/high findings

## Tools
- cargo-audit
- cargo-deny
- npm audit
- snyk
- trivy
- hadolint
- trufflehog

## Command
```bash
# Full audit
make audit

# Individual audits
make audit-rust
make audit-js
make audit-docker
make audit-secrets
```

## Output
- JSON reports per tool
- Aggregated summary
- PR check status
- Notifications (Discord/Telegram/Email)

## Success Criteria
- No critical/high vulnerabilities
- All tools pass
- Report generated

## Failure Handling
- Block merge on critical/high
- Notify security team
- Create issue for remediation
