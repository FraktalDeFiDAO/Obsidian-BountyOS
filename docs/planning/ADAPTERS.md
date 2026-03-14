# Platform Adapter Specifications

## ObsidianBountyFinder

**Version:** 1.0  
**Last Updated:** 2026-03-14

---

## Overview

Each platform adapter implements the `BountyAdapter` trait to provide a unified interface for fetching bounties from different platforms.

## Adapter Architecture

```rust
pub trait BountyAdapter: Send + Sync {
    fn platform(&self) -> Platform;
    async fn fetch_all(&self) -> Result<Vec<Bounty>, AdapterError>;
    async fn fetch_updates(&self, since: DateTime<Utc>) -> Result<Vec<Bounty>, AdapterError>;
    async fn fetch_bounty(&self, external_id: &str) -> Result<Option<Bounty>, AdapterError>;
    async fn search(&self, query: &SearchQuery) -> Result<Vec<Bounty>, AdapterError>;
    fn supports_hooks(&self) -> bool;
    fn hook_url(&self) -> Option<String>;
    fn validate_config(&self) -> Result<(), AdapterError>;
}
```

---

## GitHub Adapter

### Status: ✅ Stable

### Data Sources
- Issues with `label:bounty` or `label:help-wanted`
- Issues with funding in description
- GitHub Sponsors funded issues
- PRs with reward mentions

### API Method
- **Primary:** GitHub REST API v3
- **GraphQL:** For complex queries (requires authentication)

### Rate Limits
- Unauthenticated: 60 requests/hour
- Authenticated: 5,000 requests/hour

### Configuration
```env
GITHUB_TOKEN=ghp_xxxxxxxxxxxx
GITHUB_ORGANIZATION=optional-org-filter
```

### Implementation Details
```rust
struct GitHubAdapter {
    client: reqwest::Client,
    token: Option<String>,
    organization: Option<String>,
}

impl BountyAdapter for GitHubAdapter {
    async fn fetch_all(&self) -> Result<Vec<Bounty>, AdapterError> {
        // Fetch issues with funding labels
        // Paginate through all pages
        // Convert to unified Bounty format
    }
}
```

### Data Mapping
| GitHub Field | Bounty Field |
|--------------|--------------|
| `issue.number` | `external_id` |
| `issue.title` | `title` |
| `issue.body` | `description` |
| `issue.html_url` | `url` |
| `issue.labels` | `tags` |
| `issue.created_at` | `created_at` |
| `issue.updated_at` | `updated_at` |

---

## Gitcoin Adapter

### Status: ✅ Stable

### Data Sources
- Active grants
- Quadratic Funding rounds
- Hackathon projects
- Bounties (sybil-protected)

### API Method
- **Primary:** Gitcoin GraphQL API

### Configuration
```env
GITCOIN_API_KEY=xxxxxxxxxxxx
```

### GraphQL Endpoint
```
https://gitcoin.co/grants/v1/graphql
```

### Data Mapping
| Gitcoin Field | Bounty Field |
|---------------|--------------|
| `grant.id` | `external_id` |
| `grant.title` | `title` |
| `grant.description` | `description` |
| `grant.url` | `url` |
| `grant.amountRaised` | `reward_min/max` |
| `grant.token` | `reward_currency` |
| `grant.tags` | `tags` |

---

## HackerOne Adapter

### Status: ✅ Stable

### Data Sources
- Active bug bounty programs
- Program scope definitions
- Asset types

### API Method
- **Primary:** HackerOne Program API

### Configuration
```env
HACKERONE_API_KEY=xxxxxxxxxxxx
HACKERONE_USERNAME=your_username
```

### API Endpoint
```
https://api.hackerone.com/api/v1
```

### Data Mapping
| HackerOne Field | Bounty Field |
|----------------|--------------|
| `program.id` | `external_id` |
| `program.name` | `title` |
| `program.description` | `description` |
| `program.url` | `url` |
| `program.structured_scopes` | `metadata` |
| `program.created_at` | `created_at` |

---

## Bugcrowd Adapter

### Status: ✅ Stable

### Data Sources
- Active programs
- VRT (Vulnerability Rating Taxonomy)
- Scope definitions

### API Method
- **Primary:** Bugcrowd API v2

### Configuration
```env
BUGCROWD_API_KEY=xxxxxxxxxxxx
BUGCROWD_USERNAME=your_username
```

### API Endpoint
```
https://api.bugcrowd.com/v2
```

### Data Mapping
| Bugcrowd Field | Bounty Field |
|----------------|--------------|
| `program.name` | `title` |
| `program.description` | `description` |
| `program.targets` | `metadata` |

---

## LaborX Adapter

### Status: 🔄 Beta

### Data Sources
- Time-based jobs
- Milestone tasks
- Hourly/fixed-price projects

### API Method
- **Primary:** LaborX API
- **Fallback:** Web scraping

### Configuration
```env
LABORX_API_KEY=xxxxxxxxxxxx
```

### Notes
- May require scraping for complete data
- Rate limits apply
- Some data may require authentication

---

## DeWork Adapter

### Status: 🔄 Beta

### Data Sources
- Active tasks
- Payout information
- Task requirements

### API Method
- **Primary:** DeWork GraphQL API

### Configuration
```env
DEWORK_API_KEY=xxxxxxxxxxxx
```

### GraphQL Endpoint
```
https://api.dework.com/graphql
```

---

## Error Handling

### Retry Strategy
```rust
async fn fetch_with_retry<F, T, E>(mut f: F) -> Result<T, AdapterError>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut retries = 0;
    let max_retries = 3;
    
    loop {
        match f.await {
            Ok(result) => return Ok(result),
            Err(e) if retries < max_retries => {
                retries += 1;
                let delay = Duration::from_secs(2u64.pow(retries));
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(AdapterError::Network(e.to_string())),
        }
    }
}
```

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Rate limited, retry after: {0}")]
    RateLimited(Duration),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
}
```

---

## Testing

### Mock Data
Each adapter should have mock data for testing:
- `tests/fixtures/github/issues.json`
- `tests/fixtures/gitcoin/grants.json`
- etc.

### Integration Tests
```rust
#[tokio::test]
async fn test_github_fetch_all() {
    let adapter = GitHubAdapter::new();
    let bounties = adapter.fetch_all().await;
    
    assert!(bounties.is_ok());
    // Verify structure
}
```
