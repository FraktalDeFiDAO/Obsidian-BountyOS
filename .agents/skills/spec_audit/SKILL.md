# SPEC Audit Skill

## Description
Verifies that implementation matches the specifications defined in SPEC.md and other documentation.

## Purpose
Ensure all code changes comply with the technical specification before completion. This is a critical quality gate that must pass before any PR can be merged.

## Capabilities
- Data model verification
- API schema validation
- CLI command verification
- Database schema verification
- Adapter interface verification
- UI/UX compliance checking
- Documentation consistency

## Verification Checklist

### Data Models
- [ ] Domain entities match SPEC
- [ ] Enums match SPEC
- [ ] Structs match SPEC
- [ ] Field types match SPEC

### API Specification
- [ ] GraphQL schema matches SPEC
- [ ] REST endpoints match SPEC
- [ ] Query/Mutation names match SPEC
- [ ] Response types match SPEC

### CLI Commands
- [ ] Command structure matches SPEC
- [ ] Flags/options match SPEC
- [ ] Output format matches SPEC

### Database Schema
- [ ] Tables match SPEC
- [ ] Columns match SPEC
- [ ] Indexes match SPEC
- [ ] Foreign keys match SPEC

### Platform Adapters
- [ ] BountyAdapter trait matches SPEC
- [ ] Error types match SPEC
- [ ] Method signatures match SPEC

### Notifications
- [ ] Channels match SPEC
- [ ] Event types match SPEC
- [ ] Webhook payloads match SPEC

### Security
- [ ] Auth matches SPEC
- [ ] Rate limiting matches SPEC
- [ ] Encryption matches SPEC

## Commands
```bash
# Run SPEC audit
make spec-audit

# Verify specific component
make spec-audit COMPONENT=api
make spec-audit COMPONENT=cli
make spec-audit COMPONENT=adapters

# Verify database
make spec-audit COMPONENT=db

# Verify all
make spec-audit COMPONENT=all
```

## Triggers
- Pull request opened
- Pull request synchronized
- Before merge (required)
- Manual trigger

## Severity Levels
| Issue Type | Severity | Action |
|------------|----------|--------|
| Missing required field | Critical | Block |
| Type mismatch | Critical | Block |
| Missing endpoint | High | Block |
| Missing command | High | Block |
| Documentation mismatch | Medium | Warn |
| Naming inconsistency | Low | Suggest |

## Output
- SPEC compliance report
- List of discrepancies
- Recommendations for fixes
- Pass/Fail status

## Integration
This skill is automatically invoked by:
- All specialized agents (rust-backend, vue-frontend, platform-adapter, devops)
- PR review workflow
- Pre-merge gate

## Success Criteria
- All critical checks pass
- No high-severity issues
- Documentation updated if needed
- Report generated
