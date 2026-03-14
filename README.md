# ObsidianBountyFinder

<p align="center">
  <img src="https://img.shields.io/badge/Rust-DEA584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Vue.js-4FC08D?style=for-the-badge&logo=vuedotjs&logoColor=white" alt="Vue.js">
  <img src="https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white" alt="Tailwind">
  <img src="https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL">
</p>

> Unified bounty and opportunity tracker for developers. Monitor GitHub, Gitcoin, HackerOne, Bugcrowd, LaborX, DeWork and more - all from one CLI.

## Features

- **CLI-First Design** - Scan and track bounties from your terminal
- **Multi-Platform Support** - GitHub, Gitcoin, HackerOne, Bugcrowd, LaborX, DeWork
- **Real-Time Notifications** - Telegram, Discord, Email, System notifications
- **Interactive TUI** - Beautiful terminal interface with filters
- **Web Interface** - Vue3 + Tailwind4 web dashboard
- **Mobile App** - Tauri Mobile for iOS/Android
- **Multi-Chain Wallet** - BTC, ETH, SOL, DOGE, LTC support
- **Webhook Server** - Real-time updates via WebSocket + HTTP callbacks
- **Enterprise-Ready** - PostgreSQL, Docker, CI/CD, Security Audits

## Quick Start

### Prerequisites

- Rust 1.80+ (install via [rustup](https://rustup.rs))
- Node.js 22+ (for web frontend)
- PostgreSQL 16+ (optional, SQLite works for standalone)

### Installation

```bash
# Clone the repository
git clone https://github.com/obsidian-bounty-finder/obsidian-bounty-finder.git
cd obsidian-bounty-finder

# Copy environment configuration
cp .env.example .env

# Build the CLI
cargo build --release

# Run initial scan
./target/release/obsidian-bounty-finder scan --all
```

### CLI Usage

```bash
# Scan specific platform
obsidian-bounty-finder scan --platform github

# List all bounties
obsidian-bounty-finder list --status active

# Start API server
obsidian-bounty-finder serve --port 8080

# Enable notifications
obsidian-bounty-finder notify --channel telegram --token YOUR_TOKEN

# Rescan all platforms (full sync)
obsidian-bounty-finder sync --all --force
```

### Docker Compose

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    OBSIDIAN BOUNTY FINDER                   │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────────┐   │
│  │  CLI (Rust) │   │ API Server  │   │  Web (Vue3)    │   │
│  │  Scanner    │◄─►│ GraphQL     │◄─►│  Dashboard      │   │
│  └─────────────┘   └──────┬──────┘   └─────────────────┘   │
│                          │                                  │
│  ┌───────────────────────┴───────────────────────────────┐  │
│  │              Platform Adapters (Rust)                 │  │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌───────┐ ┌─────┐ │  │
│  │  │GitHub  │ │Gitcoin │ │Hacker1 │ │Bugcrowd│ │More │ │  │
│  │  └────────┘ └────────┘ └────────┘ └───────┘ └─────┘ │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Supported Platforms

| Platform | Status | Method |
|----------|--------|--------|
| GitHub | ✅ Stable | API + Issues Search |
| Gitcoin | ✅ Stable | GraphQL API |
| HackerOne | ✅ Stable | Program API |
| Bugcrowd | ✅ Stable | API |
| LaborX | 🔄 Beta | API + Scrape |
| DeWork | 🔄 Beta | GraphQL API |

## Configuration

See `.env.example` for all configuration options.

```bash
# Required: At least one platform API token
GITHUB_TOKEN=ghp_xxxx
GITCOIN_API_KEY=xxxx
HACKERONE_API_KEY=xxxx
```

## Development

```bash
# Install dependencies
make setup

# Run tests
make test

# Run lints
make lint

# Start development server
make dev

# Run with act (local CI)
make ci
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Backend | Rust (2021), Actix-web, Async-graphql |
| CLI | Ratatui, Crossterm, Clap |
| Database | PostgreSQL, SQLite |
| Frontend | Vue 3, Tailwind 4, Pinia, Viem |
| Mobile | Tauri 2.x |
| Containers | Docker, Podman |
| CI/CD | GitHub Actions, act |

## Security

This project undergoes regular security audits:

- `cargo-audit` - Dependency vulnerabilities
- `cargo-deny` - License compliance
- `trufflehog` - Secret detection
- `trivy` - Container scanning
- `npm audit` - JS dependency scanning

## Donations

Support the project by donating to any of these wallets:

| Network | Address | Label |
|---------|---------|-------|
| Ethereum | `0x0e4c337F1b053F41a0d8CE1d553A997df18Be7af` | Main ETH |
| Bitcoin | `bc1qg9xj44mya6h6y67w82aw5lqt0rzm7qfsnm4egn` | Main BTC |
| Solana | `FH84Dg6gh7bWtyZ5a1SBNLp1JBesLoCKx9mekJpr7zHR` | Main SOL |
| TRON | `TKrZ6Bu36zaVudYWPRcjZTAkTJvK1X7tXa` | Main TRON |
| RustChain | `RTCbc57f8031699a0bab6e9a8a2769822f19f115dc5` | Main RTC |

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

---

**Built with ❤️ for the bounty hunting community**
