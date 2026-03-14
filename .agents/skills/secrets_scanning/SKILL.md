# Secrets Scanning Skill

## Description
Detects exposed secrets, API keys, passwords, and other sensitive information in the codebase.

## Capabilities
- Scan for API keys
- Detect password leaks
- Find private keys
- Identify credentials in code

## Tools
- trufflehog
- git-secrets
- gitleaks
- secretlint

## File Patterns
- All source files
- `.env*` files
- Configuration files
- Git history

## Commands
```bash
# Scan filesystem
trufflehog filesystem .

# Scan with JSON output
trufflehog filesystem . --json > secrets.json

# Scan git history
trufflehog origin

# GitLeaks scan
gitleaks detect --source .

# Secretlint
secretlint "**/*"
```

## Triggers
- Pull requests
- Push to main/develop
- Daily scheduled run
- Before commits (optional)

## Excluded Paths
- `.git/`
- `node_modules/`
- `target/`
- `coverage/`
- `*.test.*`
- `*.spec.*`

## Findings Categories
- AWS keys
- GitHub tokens
- Private keys
- Database passwords
- API keys
- Generic secrets

## Action on Finding
1. Block the build
2. Report in PR
3. Notify security channel
4. Provide remediation steps

## Output
- JSON report
- PR check failure
- Detailed findings with locations
