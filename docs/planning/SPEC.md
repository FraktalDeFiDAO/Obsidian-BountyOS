# Technical Specification (SPEC)

## ObsidianBountyFinder

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2026-03-14

---

## 1. Architecture Overview

### 1.1 System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           OBSIDIAN BOUNTY FINDER                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                         CLI (Rust - Primary)                          │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────────┐  │   │
│  │  │   Scanner  │  │    Sync    │  │  Notifier  │  │ Webhook Svr  │  │   │
│  │  │   Engine   │  │  Manager    │  │            │  │ (WS + HTTP)  │  │   │
│  │  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘  └──────┬───────┘  │   │
│  │        │               │               │                │          │   │
│  │        └───────────────┼───────────────┴────────────────┘          │   │
│  │                        │                                              │   │
│  │                  ┌─────▼─────┐                                        │   │
│  │                  │  Database │                                        │   │
│  │                  │ (SQLite/   │                                        │   │
│  │                  │  Postgres) │                                        │   │
│  │                  └─────┬─────┘                                        │   │
│  └────────────────────────┼────────────────────────────────────────────┘   │
│                           │                                                 │
│              ┌─────────────┴─────────────┐                                 │
│              │      API Server (Rust)    │                                 │
│              │   (GraphQL + REST)         │                                 │
│              │   - Connects to CLI data  │                                 │
│              │   - Serves Web/Mobile      │                                 │
│              └─────────────┬─────────────┘                                 │
│                              │                                              │
│     ┌───────────────────────┼───────────────────────┐                      │
│     │                       │                       │                      │
│ ┌───▼────┐           ┌──────▼──────┐         ┌──────▼──────┐              │
│ │  Web   │           │   Mobile    │         │   Others   │              │
│ │ (Vue3) │           │  (Tauri)     │         │  (API)      │              │
│ └────────┘           └─────────────┘         └─────────────┘              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.2 CLI-First Architecture

**Two Modes of Operation:**

| Mode | Database | API Server | Use Case |
|------|----------|------------|----------|
| **Standalone** | SQLite (local) | ❌ Disabled | CLI-only usage |
| **Server** | PostgreSQL | ✅ Enabled | Full stack |

**Mode Switching:**
- Default: Standalone (SQLite in `./data/`)
- Server mode: `--server` flag or `APP_MODE=server`

---

## 2. Data Models

### 2.1 Core Entities

```rust
// Domain: Bounty
pub struct Bounty {
    pub id: Uuid,
    pub external_id: String,
    pub platform: Platform,
    pub title: String,
    pub description: String,
    pub url: String,
    pub bounty_type: BountyType,
    pub status: BountyStatus,
    pub reward_min: Option<Decimal>,
    pub reward_max: Option<Decimal>,
    pub reward_currency: Option<String>,
    pub skills: Vec<String>,
    pub tags: Vec<String>,
    pub metadata: Json<PlatformMetadata>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub synced_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub enum Platform {
    GitHub,
    Gitcoin,
    HackerOne,
    Bugcrowd,
    LaborX,
    DeWork,
    Custom(String),
}

pub enum BountyType {
    BugBounty,
    Task,
    Grant,
    Microtask,
    Hackathon,
    Bounty,
}

pub enum BountyStatus {
    Active,
    Closed,
    Expired,
    Draft,
    Paused,
}
```

---

## 3. Adapter Architecture

### 3.1 Adapter Trait

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

### 3.2 Platform Adapters

| Platform | Method | Priority |
|----------|--------|----------|
| GitHub | REST API + GraphQL | P0 |
| Gitcoin | GraphQL API | P0 |
| HackerOne | Program API | P1 |
| Bugcrowd | API | P1 |
| LaborX | API + Scrape | P2 |
| DeWork | GraphQL API | P2 |

---

## 4. API Specification

### 4.1 GraphQL Schema

```graphql
type Query {
    bounties(
        platform: PlatformFilter,
        status: StatusFilter,
        bountyType: BountyTypeFilter,
        minReward: Float,
        maxReward: Float,
        skills: [String!],
        search: String,
        limit: Int = 50,
        offset: Int = 0
    ): BountyConnection!
    
    bounty(id: ID!): Bounty
    bountyByPlatform(platform: Platform!, externalId: String!): Bounty
    platforms: [PlatformInfo!]!
    platformStatus(platform: Platform!): PlatformStatus!
    syncHistory(platform: Platform, limit: Int = 10): [SyncHistory!]!
    metrics(platform: Platform, from: DateTime, to: DateTime): [Metric!]!
    health: Health!
}

type Mutation {
    syncPlatform(platform: Platform!): SyncResult!
    syncAllPlatforms: [SyncResult!]!
    updatePreferences(input: PreferencesInput!): UserPreferences!
    registerWebhook(input: WebhookInput!): Webhook!
    deleteWebhook(id: ID!): Boolean!
    updatePlatformConfig(input: PlatformConfigInput!): PlatformConfig!
}

type Subscription {
    bountyAdded(platform: Platform): Bounty!
    bountyUpdated(platform: Platform): Bounty!
    bountyClosed(platform: Platform): Bounty!
    syncCompleted(platform: Platform): SyncHistory!
}
```

### 4.2 REST Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/ready` | Readiness check |
| GET | `/api/v1/bounties` | List bounties |
| GET | `/api/v1/bounties/:id` | Get bounty |
| POST | `/api/v1/webhooks` | Register webhook |
| DELETE | `/api/v1/webhooks/:id` | Delete webhook |
| POST | `/api/v1/sync` | Trigger sync |

---

## 5. CLI Specification

### 5.1 Commands

```
obsidian-bounty-finder scan --platform github --all --force
obsidian-bounty-finder list --status active --limit 50
obsidian-bounty-finder serve --host 127.0.0.1 --port 8080 --server
obsidian-bounty-finder notify --channel telegram --test
obsidian-bounty-finder sync --all
obsidian-bounty-finder config --set api.github_token=xxx
obsidian-bounty-finder status
```

---

## 6. Database Schema

### PostgreSQL Tables

- `bounties` - Core bounty data
- `platforms` - Platform configuration
- `sync_history` - Sync records
- `user_preferences` - User settings
- `webhooks` - Webhook registrations
- `notifications` - Notification log
- `metrics` - System metrics

---

## 7. Notifications

| Channel | Implementation |
|---------|----------------|
| System | notify-rust crate |
| Telegram | Bot API |
| Discord | Webhooks |
| Email | SMTP (lettre) |
| Webhook | HTTP POST |

---

## 8. Security

- JWT tokens (15-min expiry)
- API key authentication for webhooks
- AES-256-GCM encryption for secrets
- Rate limiting (100/min GraphQL, 200/min REST)

---

## 9. Configuration

Configuration precedence (highest to lowest):
1. Environment variables
2. `.env` file
3. `config.toml`
4. Default values
