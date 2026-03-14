# JavaScript/TypeScript Security Audit Skill

## Description
Performs security audits on JavaScript/TypeScript dependencies using npm audit and Snyk.

## Capabilities
- Detect vulnerable npm packages
- Check for known CVEs
- License compliance
- Generate security reports

## Tools
- npm audit
- snyk
- npm ls

## File Patterns
- `package.json`
- `package-lock.json`
- `npm-shrinkwrap.json`
- `frontend/**`

## Commands
```bash
# Basic audit
npm audit

# Audit with level
npm audit --audit-level=moderate

# JSON output
npm audit --json > audit.json

# Snyk test
snyk test

# Snyk monitor
snyk monitor
```

## Triggers
- Daily scheduled run
- Pull requests
- Push to main/develop

## Severity Levels
| Level | npm audit | Snyk |
|-------|-----------|------|
| Critical | exit 1 | Critical |
| High | exit 1 | High |
| Moderate | exit 0 | Medium |
| Low | exit 0 | Low |

## Auto-fix
- `npm audit fix` for auto-fixable issues
- `snyk wizard` for Snyk recommendations

## Output
- JSON report
- PR check summary
- Notifications to Discord/Telegram/Email
