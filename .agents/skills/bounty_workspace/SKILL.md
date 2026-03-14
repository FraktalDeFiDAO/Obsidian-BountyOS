# Bounty Workspace Skill

## Description

Prepare local workspace with all specifications, requirements, and testing infrastructure.

## Workspace Structure

```
bounties/{platform}/{bounty-id}/
├── SPEC.md                    # Full bounty specification
├── requirements.md            # Research findings
├── payment-checklist.md      # Payment verification checklist
├── README.md                 # Quick reference
├── workspace/
│   ├── Dockerfile            # Podman/Docker environment
│   ├── docker-compose.yml   # Multi-service setup
│   ├── .env.example         # Environment template
│   ├── tests/               # Testing suite
│   │   ├── integration/
│   │   ├── unit/
│   │   └── prompts/         # AI analysis prompts
│   ├── notes/               # Research notes
│   └── scripts/             # Automation scripts
└── reports/                # Submission reports
```

## Dockerfile Best Practices

- Use minimal base images
- Include all tools needed for testing
- Support both podman and docker
- Include debugging tools
- Document all dependencies

## Payment Checklist Template

- [ ] Eligibility confirmed
- [ ] Scope verified
- [ ] Reproduction steps documented
- [ ] PoC created and tested
- [ ] Severity assessment complete
- [ ] Report format validated
- [ ] Disclosure terms understood
- [ ] Submission timestamp noted
- [ ] Follow-up plan in place

## Testing Suite Types

1. **Integration Tests**: Full workflow testing
2. **Unit Tests**: Component-level verification
3. **AI Prompts**: Analysis prompts for AI tools
4. **Automation Scripts**: Repeatable verification

## Usage

```
bounty-workspace create --platform github --id 123 --title "Fix bug"
```
