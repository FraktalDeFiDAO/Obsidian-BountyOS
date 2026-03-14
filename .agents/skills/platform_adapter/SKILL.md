# Platform Adapter Skill

## Description
Specialized skill for developing platform adapters that fetch bounties from various platforms.

## Capabilities
- Implement BountyAdapter trait
- API integration (REST/GraphQL)
- Web scraping with reqwest
- Rate limiting handling
- Error handling and retries
- Data normalization

## Platforms Supported
- GitHub
- Gitcoin
- HackerOne
- Bugcrowd
- LaborX
- DeWork

## Tools
- reqwest (HTTP client)
- scraper (HTML parsing)
- serde (serialization)
- tokio (async runtime)

## File Patterns
- `src/adapters/**`
- `src/adapters/**/mod.rs`
- `src/adapters/**/tests/**`

## Adapter Template

```rust
pub struct PlatformAdapter {
    client: reqwest::Client,
    config: AdapterConfig,
}

impl BountyAdapter for PlatformAdapter {
    fn platform(&self) -> Platform {
        Platform::PlatformName
    }

    async fn fetch_all(&self) -> Result<Vec<Bounty>, AdapterError> {
        // Implementation
    }

    async fn fetch_updates(&self, since: DateTime<Utc>) -> Result<Vec<Bounty>, AdapterError> {
        // Implementation
    }

    fn supports_hooks(&self) -> bool {
        // Return true if platform supports webhooks
    }
}
```

## Error Handling
```rust
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Rate limited")]
    RateLimited,
    
    #[error("Authentication error")]
    Auth(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
}
```

## Testing
- Mock HTTP responses
- Fixture files
- Integration tests
- Error case handling

## Triggers
- Changes to adapter files
- New platform requirements
- API changes
