# DevOps Skill

## Description
Specialized skill for CI/CD, infrastructure, containerization, and deployment automation.

## Capabilities
- GitHub Actions workflows
- Docker/Podman containerization
- Kubernetes deployment (optional)
- Infrastructure as Code
- Environment management
- Release management

## Tools
- GitHub Actions
- Docker / Podman
- kubectl (optional)
- Terraform (optional)
- act (local CI)

## File Patterns
- `.github/workflows/**`
- `Dockerfile*`
- `docker-compose*.yml`
- `.gitlab-ci.yml`
- `kubernetes/**`

## Workflow Files

### CI Pipeline (ci.yml)
```yaml
# Triggers: PR, push
Jobs:
  - lint (rustfmt, clippy, eslint)
  - test (cargo test, vitest)
  - build (cargo build, npm build)
  - security (audit, scan)
```

### CD Pipeline (cd.yml)
```yaml
# Triggers: Tag (v*)
Jobs:
  - build (Docker image)
  - deploy-staging
  - deploy-production
```

### Security Pipeline (security.yml)
```yaml
# Triggers: PR, push, daily schedule
Jobs:
  - rust-audit
  - npm-audit
  - trivy
  - trufflehog
```

## Commands

### Local CI
```bash
# Run PR workflow
act pull_request

# Run specific job
act -j lint-rust

# Run all workflows
act
```

### Docker
```bash
# Build image
docker build -t obsidian-bounty-finder .

# Run container
docker run -d -p 8080:8080 obsidian-bounty-finder

# Docker Compose
docker-compose up -d
```

### Kubernetes (optional)
```bash
# Deploy
kubectl apply -f kubernetes/

# Check status
kubectl get pods
```

## Environment Configuration
- Development (local)
- Staging
- Production

## Secrets Management
- GitHub Secrets
- Environment variables
- .env files (not committed)

## Triggers
- Changes to CI/CD files
- Docker configuration changes
- Infrastructure changes
- Release tags

## Best Practices
- Immutable tags
- Multi-stage Docker builds
- Layer caching
- Security scanning in CI
- Automated rollbacks
