# Docker Security Audit Skill

## Description
Performs security scans on Docker images and Dockerfiles.

## Capabilities
- Scan Docker images for vulnerabilities
- Lint Dockerfiles for best practices
- Check base image security
- Identify exposed secrets

## Tools
- trivy
- hadolint
- docker bench

## File Patterns
- `Dockerfile*`
- `docker-compose*.yml`
- `.dockerignore`

## Commands
```bash
# Scan filesystem
trivy fs --severity HIGH,CRITICAL .

# Scan Docker image
trivy image myimage:latest

# Scan with JSON output
trivy fs --format json --output results.json .

# Lint Dockerfile
hadolint Dockerfile

# Docker Bench Security
docker bench security
```

## Triggers
- Daily scheduled run
- Pull requests
- Push to main/develop

## Severity Levels
| Severity | Description |
|----------|-------------|
| CRITICAL | RCE, data breach |
| HIGH | Privilege escalation |
| MEDIUM | Information disclosure |
| LOW | Best practice violation |

## Best Practices Check
- No root user
- Minimal base image
- No secrets in image
- Multi-stage builds
- Proper layer caching

## Output
- JSON/table report
- PR check summary
- Hadolint violations
