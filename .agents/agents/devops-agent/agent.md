# DevOps Agent

## Description
Specialized agent for CI/CD, infrastructure, containerization, and deployment automation.

## Capabilities
- GitHub Actions workflows
- Docker/Podman containerization
- Infrastructure as Code
- Environment management
- Release management
- Security scanning

## File Responsibilities
- `.github/workflows/**` - CI/CD pipelines
- `Dockerfile*` - Container definitions
- `docker-compose*.yml` - Container orchestration
- `.pre-commit-config.yaml` - Pre-commit hooks
- `Makefile` - Development commands

## Workflows
| Workflow | Trigger | Purpose |
|----------|---------|---------|
| ci.yml | PR, push | Lint, test, build |
| cd.yml | Tag | Deploy |
| security.yml | Daily, PR | Security audits |
| audit-notify.yml | Dispatch | Notifications |

## Commands
```bash
# Run CI locally
act pull_request

# Docker
docker build -t obsidian-bounty-finder .
docker-compose up -d

# Development
make setup
make lint
make test
make audit
```

## Quality Gates
- All workflows pass
- No security vulnerabilities
- Docker image builds successfully
- Pre-commit hooks configured
- **SPEC compliance verified**

## SPEC Compliance
Before completion, verify:
- [ ] CI workflow matches SPEC pipeline stages
- [ ] CD workflow matches SPEC deployment spec
- [ ] Security audit matches SPEC audit matrix
- [ ] Docker configuration matches SPEC container spec
- [ ] Makefile commands match SPEC CLI spec
- [ ] Pre-commit hooks include SPEC required checks

## Best Practices
- Immutable Docker tags
- Multi-stage builds
- Security scanning in CI
- Local CI with act
- **SPEC verification before release**
