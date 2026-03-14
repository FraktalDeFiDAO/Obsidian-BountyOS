# Platform Adapter Agent

## Description
Specialized agent for developing and maintaining platform adapters that fetch bounties from various platforms.

## Capabilities
- BountyAdapter trait implementation
- REST API integration
- GraphQL API integration
- Web scraping
- Rate limiting
- Error handling and retries
- Data normalization

## Supported Platforms
- GitHub (REST + GraphQL)
- Gitcoin (GraphQL)
- HackerOne (API)
- Bugcrowd (API)
- LaborX (API + scrape)
- DeWork (GraphQL)

## File Responsibilities
- `src/adapters/**` - All adapter implementations
- `src/adapters/*/src/**` - Adapter source code
- `src/adapters/*/tests/**` - Adapter tests
- `docs/planning/ADAPTERS.md` - Documentation

## Adapter Structure
```
src/adapters/
├── Cargo.toml
├── src/
│   ├── lib.rs (trait definition)
│   ├── github.rs
│   ├── gitcoin.rs
│   ├── hackerone.rs
│   ├── bugcrowd.rs
│   ├── laborx.rs
│   └── dework.rs
└── tests/
    ├── fixtures/
    └── integration_test.rs
```

## Quality Gates
- All adapters implement BountyAdapter trait
- Tests for each adapter
- Mock data for testing
- Error handling for all API errors
- Rate limiting handled

## Workflow Integration
- Triggered on changes to adapters
- Runs adapter-specific tests
- Validates API integration
