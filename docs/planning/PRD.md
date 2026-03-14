# Product Requirements Document (PRD)

## ObsidianBountyFinder

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2026-03-14

---

## 1. Executive Summary

### 1.1 Vision
Create a unified platform to monitor, track, and manage freelance opportunities, bounties, and microtasks across multiple platforms (GitHub, Gitcoin, bug bounty sites, paid task platforms) from a single CLI-first interface.

### 1.2 Mission
Aggregate all bounty and freelance opportunities in one place, enabling developers to find paid work faster with real-time notifications and multi-platform search.

### 1.3 Problem Statement
- Developers must check multiple platforms individually to find work
- No unified notification system for new opportunities
- Difficulty tracking application status across platforms
- Lack of standardized data format across platforms

---

## 2. User Personas

### 2.1 Bug Bounty Hunter
**Name:** Alex  
**Background:** Security researcher, 5 years experience  
**Goals:**
- Find high-paying bug bounties quickly
- Filter by program scope and reward
- Get notified immediately of new programs

**Pain Points:**
- Manually checking HackerOne, Bugcrowd daily
- Missing time-limited opportunities

### 2.2 Freelance Developer
**Name:** Jordan  
**Background:** Full-stack developer, gig economy  
**Goals:**
- Find short-term paid tasks
- Filter by required skills
- Track multiple applications

**Pain Points:**
- Tasks scattered across different platforms

### 2.3 Open Source Contributor
**Name:** Sam  
**Background:** Open source maintainer  
**Goals:**
- Find funded issues to work on
- Verify grant opportunities
- Track sponsored work

**Pain Points:**
- GitHub issues buried in repos
- No visibility into funding

### 2.4 DeFi Yield Farmer
**Name:** Casey  
**Background:** DeFi enthusiast  
**Goals:**
- Find protocol grants
- Track Quadratic Funding rounds
- Monitor hackathon prizes

**Pain Points:**
- Gitcoin rounds hard to track
- Multiple grant platforms

---

## 3. Core Features

### 3.1 Priority Matrix

| Priority | Feature | Description | User Value |
|----------|---------|-------------|------------|
| **P0** | CLI Scanner | Rust CLI with TUI + command flags | Core product |
| **P0** | Platform Adapters | GitHub, Gitcoin, HackerOne, Bugcrowd, LaborX, DeWork | Data sources |
| **P0** | Local Database | SQLite for standalone mode | Data persistence |
| **P0** | Full Sync | Initial scan of all platforms | Data population |
| **P0** | Rescan | Full re-scan option | Data refresh |
| **P1** | Incremental Sync | Delta updates only | Efficiency |
| **P1** | Notifications | System, Telegram, Discord, Email | User awareness |
| **P1** | Webhook Server | WS + HTTP callbacks | Real-time updates |
| **P1** | Filtering | By platform, reward, skills, status | Searchability |
| **P2** | Web UI | Vue3 + Tailwind4 dashboard | Accessibility |
| **P2** | API Server | GraphQL + REST | Extensibility |
| **P2** | Search | Full-text search across bounties | Discoverability |
| **P3** | Mobile App | Tauri Mobile | On-the-go access |
| **P3** | Wallet | Multi-chain (BTC, ETH, SOL, DOGE) | Payment tracking |
| **P3** | Analytics | Charts, trends, success rates | Insights |

### 3.2 Feature Descriptions

#### P0 Features

**CLI Scanner**
- Hybrid interface: Interactive TUI + command flags
- Commands: `scan`, `sync`, `list`, `notify`, `serve`, `config`
- Colors, real-time updates, keyboard navigation
- Offline mode support

**Platform Adapters**
- Each platform has dedicated adapter crate
- Unified `BountyAdapter` trait
- Handles API, CLI, scraping as needed
- Graceful degradation when platform unavailable

**Database**
- SQLite for standalone mode
- PostgreSQL for server mode
- Schema versioning with migrations
- Full-text search support

**Full Sync (onInit)**
- Scan ALL active bounties on first run
- Handle pagination for large datasets
- Store complete bounty data
- Track sync history

**Rescan**
- Force complete re-scan of any/all platforms
- Compare with existing data
- Update changed bounties
- Mark deleted bounties

#### P1 Features

**Incremental Sync (onUpdate)**
- Poll at configurable intervals (default: 15 min)
- Use platform hooks when available
- Hash comparison for platforms without hooks
- Only fetch changes since last sync

**Notifications**
- System: Native OS notifications
- Telegram: Bot API with inline keyboards
- Discord: Webhooks with embeds
- Email: SMTP with templates
- Configurable per-platform filters

**Webhook Server**
- WebSocket: Real-time pushes to clients
- HTTP: POST to registered URLs
- Event types: `bounty.new`, `bounty.updated`, `bounty.closed`
- Authentication via API keys

#### P2 Features

**Web UI**
- Vue3 + Tailwind4
- Mobile-first design
- Dark/light theme
- Real-time updates via WebSocket

**API Server**
- GraphQL: Complex queries, subscriptions
- REST: Health, webhooks, simple endpoints
- JWT authentication
- Rate limiting

#### P3 Features

**Mobile App**
- Tauri Mobile (iOS + Android)
- Shares Rust core with CLI
- Push notifications
- Offline-first architecture

**Wallet Integration**
- viem for EVM chains
- BTC, LTC, DOGE support
- Solana via web3.js adapter
- Balance display, transaction history

---

## 4. User Stories

### 4.1 CLI User Stories

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-001 | As a user, I want to scan GitHub for funded issues | CLI fetches all issues with bounty labels |
| US-002 | As a user, I want to filter by reward amount | CLI shows min/max rewards, filters work |
| US-003 | As a user, I want to enable Telegram notifications | Bot sends new bounty alerts to chat |
| US-004 | As a user, I want to start the API server | Server starts, responds to health check |
| US-005 | As a user, I want to rescan all platforms | Full sync completes, counts shown |

### 4.2 Web UI User Stories

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-101 | As a user, I want to view all bounties in a list | List renders with pagination |
| US-102 | As a user, I want to filter by platform | Checkbox filters work |
| US-103 | As a user, I want to view bounty details | Modal/page shows full info |
| US-104 | As a user, I want to connect my wallet | Wallet connects, balance shows |

### 4.3 Platform Adapter User Stories

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-201 | As a user, I want GitHub issues to sync | All funded issues appear in database |
| US-202 | As a user, I want Gitcoin grants to sync | Active grants appear with funding info |
| US-203 | As a user, I want HackerOne programs to sync | Active programs with scope appear |

---

## 5. Success Metrics

### 5.1 Technical Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Time to first scan | < 5 min | Manual test |
| Notification latency | < 30 sec | Log timestamps |
| API response time | < 200ms | P95 latency |
| Sync interval | 15 min (configurable) | Cron schedule |
| Uptime | 99.9% | Monitoring |

### 5.2 User Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Active users | TBD | DAU |
| Platforms covered | 6 | Count |
| Bounties indexed | 10,000+ | DB count |
| Notification delivery | 99% | Success rate |

---

## 6. Non-Functional Requirements

### 6.1 Performance
- CLI startup: < 1 second
- Database queries: < 100ms
- Memory usage: < 100MB idle

### 6.2 Scalability
- Support 100,000+ bounties in database
- Handle 1000+ concurrent API connections
- Horizontal scaling via read replicas

### 6.3 Security
- No secrets in code
- API keys encrypted at rest
- JWT with short expiry
- Rate limiting on all endpoints

### 6.4 Reliability
- Graceful degradation when platform down
- Retry logic with exponential backoff
- Idempotent sync operations
- Data validation on ingest

---

## 7. Out of Scope (v1.0)

- Payment processing
- Escrow services
- Dispute resolution
- User authentication (beyond API keys)
- Mobile push notifications (APNs/FCM)
- Desktop app (Tauri Desktop)

---

## 8. Timeline

| Week | Milestone |
|------|-----------|
| 1 | Foundation, CI/CD, .agents |
| 2 | Core backend, 2 adapters |
| 3 | Remaining adapters |
| 4 | CLI, notifications, webhooks |
| 5 | Web UI |
| 6 | Mobile, wallet |
| 7 | Polish, E2E tests |

---

## 9. Dependencies

### 9.1 External Services

| Service | Purpose | Free Tier |
|---------|---------|------------|
| GitHub API | Issue data | 5000/hr |
| Gitcoin | Grants, bounties | Yes |
| HackerOne | Bug bounties | Limited |
| Bugcrowd | Bug bounties | Limited |
| LaborX | Paid tasks | Yes |
| DeWork | Tasks | Yes |
| Telegram | Notifications | Yes |
| Discord | Notifications | Yes |

### 9.2 Internal Dependencies

- CLI → Database
- CLI → Adapters
- API → Database
- API → CLI (optional)
- Web → API

---

## 10. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-------------|
| Platform API changes | High | Adapter abstraction, versioning |
| Rate limiting | Medium | Exponential backoff, caching |
| Data quality | Medium | Validation, fallback scraping |
| Platform shutdown | High | Extensible adapter architecture |
| Security vulnerabilities | High | Regular audits, dependency updates |
