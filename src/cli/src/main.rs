use clap::{Parser, Subcommand};
use obsidian_adapters::{
    AdapterRegistry, BugcrowdAdapter, DeWorkAdapter, GitHubAdapter, GitcoinAdapter,
    HackerOneAdapter, LaborXAdapter,
};
use obsidian_db::{BountyRepository, Database};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "obsidian-bounty-finder")]
#[command(version = "0.1.0")]
#[command(about = "Unified bounty and opportunity tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan platforms for bounties
    Scan {
        /// Platform to scan (github, gitcoin, hackerone, bugcrowd, laborx, dework)
        #[arg(short, long)]
        platform: Option<String>,

        /// Scan all platforms
        #[arg(short, long)]
        all: bool,

        /// Force full rescan
        #[arg(short, long)]
        force: bool,
    },

    /// List cached bounties
    List {
        /// Filter by platform
        #[arg(short, long)]
        platform: Option<String>,

        /// Filter by status (active, closed, expired)
        #[arg(short, long)]
        status: Option<String>,

        /// Limit number of results
        #[arg(short, long, default_value = "50")]
        limit: usize,

        /// Offset for pagination
        #[arg(short, long, default_value = "0")]
        offset: usize,
    },

    /// Start the API server
    Serve {
        /// Host to bind to
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Use PostgreSQL instead of SQLite
        #[arg(long)]
        server: bool,
    },

    /// Configure notifications
    Notify {
        /// Notification channel (telegram, discord, email)
        #[arg(short, long)]
        channel: String,

        /// Test notification
        #[arg(short, long)]
        test: bool,
    },

    /// Sync with platforms
    Sync {
        /// Platform to sync
        #[arg(short, long)]
        platform: Option<String>,

        /// Sync all platforms
        #[arg(short, long)]
        all: bool,
    },

    /// Manage configuration
    Config {
        /// Get config value
        #[arg(short, long)]
        get: Option<String>,

        /// Set config value
        #[arg(short, long)]
        set: Option<String>,

        /// List all config
        #[arg(short, long)]
        list: bool,
    },

    /// Show system status
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(match cli.log_level.to_lowercase().as_str() {
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        })
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    match cli.command {
        Commands::Scan {
            platform,
            all,
            force,
        } => {
            info!(
                "Starting scan: platform={:?}, all={}, force={}",
                platform, all, force
            );
            run_scan(platform, all, force).await?;
        }
        Commands::List {
            platform,
            status,
            limit,
            offset,
        } => {
            info!(
                "Listing bounties: platform={:?}, status={:?}",
                platform, status
            );
            run_list(platform, status, limit, offset).await?;
        }
        Commands::Serve { host, port, server } => {
            info!("Starting server: {}:{}, server_mode={}", host, port, server);
            start_server(host, port, server).await?;
        }
        Commands::Notify { channel, test } => {
            info!(
                "Configuring notifications: channel={}, test={}",
                channel, test
            );
            configure_notify(channel, test).await?;
        }
        Commands::Sync { platform, all } => {
            info!("Syncing: platform={:?}, all={}", platform, all);
            run_sync(platform, all).await?;
        }
        Commands::Config { get, set, list } => {
            run_config(get, set, list).await?;
        }
        Commands::Status => {
            show_status().await?;
        }
    }

    Ok(())
}

async fn get_database() -> Result<Database, Box<dyn std::error::Error>> {
    let database_path =
        std::env::var("DATABASE_PATH").unwrap_or_else(|_| "data/bounties.db".to_string());

    // Ensure directory exists
    if let Some(parent) = std::path::Path::new(&database_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let db = Database::new(&database_path)?;
    Ok(db)
}

async fn run_scan(
    platform: Option<String>,
    all: bool,
    _force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = get_database().await?;

    let mut registry = AdapterRegistry::new();

    let github_token = std::env::var("GITHUB_TOKEN").ok();
    let gitcoin_key = std::env::var("GITCOIN_API_KEY").ok();
    let github_org = std::env::var("GITHUB_ORGANIZATION").ok();
    let hackerone_key = std::env::var("HACKERONE_API_KEY").ok();
    let hackerone_user = std::env::var("HACKERONE_USERNAME").ok();
    let bugcrowd_key = std::env::var("BUGCROWD_API_KEY").ok();
    let bugcrowd_user = std::env::var("BUGCROWD_USERNAME").ok();
    let laborx_key = std::env::var("LABORX_API_KEY").ok();
    let dework_key = std::env::var("DEWORK_API_KEY").ok();

    // Register all adapters
    registry.register(Box::new(GitHubAdapter::new(github_token, github_org)));
    registry.register(Box::new(GitcoinAdapter::new(gitcoin_key)));
    registry.register(Box::new(HackerOneAdapter::new(
        hackerone_key,
        hackerone_user,
    )));
    registry.register(Box::new(BugcrowdAdapter::new(bugcrowd_key, bugcrowd_user)));
    registry.register(Box::new(LaborXAdapter::new(laborx_key)));
    registry.register(Box::new(DeWorkAdapter::new(dework_key)));

    let platforms_to_scan = if all {
        registry.platforms()
    } else if let Some(p) = platform {
        vec![obsidian_domain::Platform::parse(&p)]
    } else {
        registry.platforms()
    };

    for plat in platforms_to_scan {
        if let Some(adapter) = registry.get(&plat) {
            info!("Scanning platform: {:?}", plat);

            match adapter.fetch_all().await {
                Ok(bounties) => {
                    info!("Found {} bounties from {:?}", bounties.len(), plat);

                    for bounty in bounties {
                        if let Err(e) = db.upsert_bounty(&bounty).await {
                            error!("Failed to save bounty: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to fetch from {:?}: {}", plat, e);
                }
            }
        }
    }

    info!("Scan complete!");
    Ok(())
}

async fn run_list(
    _platform: Option<String>,
    _status: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = get_database().await?;

    let bounties = db.list_bounties(limit, offset).await?;

    println!(
        "\n{:^6} | {:^30} | {:^10} | {:^15}",
        "ID", "Title", "Platform", "Status"
    );
    println!("{:-<6}-+-{:-<30}-+-{:-<10}-+-{:-<15}", "", "", "", "");

    for bounty in bounties.iter().take(20) {
        let title = if bounty.title.len() > 28 {
            format!("{}...", &bounty.title[..25])
        } else {
            bounty.title.clone()
        };
        println!(
            "{:^6} | {:^30} | {:^10} | {:^15}",
            &bounty.id.to_string()[..6],
            title,
            bounty.platform.as_str(),
            bounty.status.as_str()
        );
    }

    let total = db.count_bounties().await?;
    println!("\nTotal: {} bounties", total);

    Ok(())
}

async fn start_server(
    _host: String,
    _port: u16,
    _server: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("API server functionality coming soon!");
    println!("Use 'cargo build --release' and run the binary to access the API.");
    Ok(())
}

async fn configure_notify(_channel: String, _test: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Notification configuration coming soon!");
    Ok(())
}

async fn run_sync(_platform: Option<String>, _all: bool) -> Result<(), Box<dyn std::error::Error>> {
    run_scan(None, true, true).await
}

async fn run_config(
    _get: Option<String>,
    _set: Option<String>,
    _list: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Configuration:");
    println!(
        "  GITHUB_TOKEN: {}",
        if std::env::var("GITHUB_TOKEN").is_ok() {
            "***"
        } else {
            "not set"
        }
    );
    println!(
        "  GITCOIN_API_KEY: {}",
        if std::env::var("GITCOIN_API_KEY").is_ok() {
            "***"
        } else {
            "not set"
        }
    );
    println!(
        "  DATABASE_URL: {}",
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/bounties.db".to_string())
    );
    Ok(())
}

async fn show_status() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_database().await?;

    println!("\n=== ObsidianBountyFinder Status ===\n");

    let total = db.count_bounties().await?;
    let bounties = db.list_bounties(1000, 0).await?;
    let active = bounties
        .iter()
        .filter(|b| b.status == obsidian_domain::BountyStatus::Active)
        .count();

    println!("Database:");
    println!("  Total bounties: {}", total);
    println!("  Active bounties: {}", active);

    println!("\nPlatforms:");
    println!(
        "  GitHub: {}",
        if std::env::var("GITHUB_TOKEN").is_ok() {
            "✓ Configured"
        } else {
            "○ Not configured"
        }
    );
    println!(
        "  Gitcoin: {}",
        if std::env::var("GITCOIN_API_KEY").is_ok() {
            "✓ Configured"
        } else {
            "○ Not configured"
        }
    );
    println!(
        "  HackerOne: {}",
        if std::env::var("HACKERONE_API_KEY").is_ok() {
            "✓ Configured"
        } else {
            "○ Not configured"
        }
    );
    println!(
        "  Bugcrowd: {}",
        if std::env::var("BUGCROWD_API_KEY").is_ok() {
            "✓ Configured"
        } else {
            "○ Not configured"
        }
    );
    println!(
        "  LaborX: {}",
        if std::env::var("LABORX_API_KEY").is_ok() {
            "✓ Configured"
        } else {
            "○ Not configured"
        }
    );
    println!(
        "  DeWork: {}",
        if std::env::var("DEWORK_API_KEY").is_ok() {
            "✓ Configured"
        } else {
            "○ Not configured"
        }
    );

    Ok(())
}
