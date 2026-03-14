# Rust Security Audit Skill

## Description
Performs security audits on Rust dependencies using cargo-audit and cargo-deny.

## Capabilities
- Detect vulnerable dependencies
- Check license compliance
- Identify yanked crates
- Generate audit reports

## Tools
- cargo-audit
- cargo-deny

## File Patterns
- `Cargo.toml`
- `Cargo.lock`
- `**/Cargo.lock`

## Commands
```bash
# Security audit
cargo audit

# Security audit with JSON output
cargo audit --json > audit.json

# License check
cargo deny check

# Check advisories
cargo audit --version
```

## Triggers
- Daily scheduled run
- Pull requests
- Push to main/develop

## Severity Levels
| Level | Action |
|-------|--------|
| Critical | Block + Notify |
| High | Block + Notify |
| Medium | Warn + Notify |
| Low | Pass |

## Auto-fix
- cargo-audit shows advisory, manual fix required
- Run `cargo update` to update dependencies

## Output
- JSON report with vulnerabilities
- Summary in PR check
- Notification to configured channels
