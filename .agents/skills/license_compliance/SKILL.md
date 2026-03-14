# License Compliance Skill

## Description
Ensures all dependencies have acceptable licenses and complies with open source license requirements.

## Capabilities
- License inventory
- Deny prohibited licenses
- Allow list management
- Compliance reporting

## Tools
- cargo-deny
- fossa
- license-cop

## File Patterns
- `Cargo.lock`
- `package.json`
- `package-lock.json`
- `LICENSE*`

## Commands
```bash
# Cargo deny check licenses
cargo deny check licenses

# Check advisories
cargo deny check advisories

# Fossa scan
fossa analyze

# License list
license-cop --list
```

## License Categories

### Allowed
- MIT
- Apache-2.0
- BSD-2-Clause
- BSD-3-Clause
- ISC
- MPL-2.0
- Unlicense

### Restricted
- GPL-2.0
- GPL-3.0
- AGPL-3.0
- LGPL-2.1

### Forbidden
- No license
- Proprietary (without approval)
- SSPL
- GPL-1.0

## Triggers
- Daily scheduled run
- Pull requests
- Push to main/develop

## Process
1. Scan all dependencies
2. Check license against allowlist
3. Generate report
4. Block if forbidden license
5. Notify team

## Output
- License inventory CSV/JSON
- Compliance report
- PR check summary
- Notifications for violations
