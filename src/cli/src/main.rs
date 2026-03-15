use clap::{Parser, Subcommand};
use obsidian_adapters::{
    AdapterRegistry, BugcrowdAdapter, DeWorkAdapter, GitHubAdapter, GitcoinAdapter,
    HackerOneAdapter, LaborXAdapter,
};
use obsidian_db::{BountyRepository, Database};
use std::path::PathBuf;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
use config::Config;

#[derive(Parser)]
#[command(name = "obsidian-bounty-finder")]
#[command(version = "0.1.0")]
#[command(about = "Unified bounty and opportunity tracker", long_about = None)]
struct Cli {
    /// Config file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Log level (debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
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

        /// Enable JavaScript rendering for dynamic content
        #[arg(long)]
        rendering: bool,

        /// Take screenshots of pages (implies rendering)
        #[arg(short, long)]
        screenshots: bool,
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
        postgres: bool,
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
        #[arg(short, long, value_parser = parse_key_value)]
        set: Option<(String, String)>,

        /// List all config
        #[arg(short, long)]
        list: bool,

        /// Create default config file
        #[arg(short, long)]
        init: bool,
    },

    /// Show system status
    Status,
}

fn parse_key_value(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() == 2 {
        Ok((parts[0].to_string(), parts[1].to_string()))
    } else {
        Err("Expected KEY=VALUE format".to_string())
    }
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

    let config = Config::load(&cli.config);

    match cli.command {
        Commands::Scan {
            platform,
            all,
            force,
            rendering,
            screenshots,
        } => {
            let use_rendering = rendering || config.scraper.use_rendering;
            let take_screenshots = screenshots || config.scraper.take_screenshots;
            info!(
                "Starting scan: platform={:?}, all={}, force={}, rendering={}, screenshots={}",
                platform, all, force, use_rendering, take_screenshots
            );
            run_scan(platform, all, force, use_rendering, take_screenshots, &config).await?;
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
        Commands::Serve { host, port, postgres } => {
            let db_url = if postgres {
                std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://user:pass@localhost/bounties".to_string())
            } else {
                config.database.url.clone()
            };
            info!("Starting server: {}:{}, db={}", host, port, db_url);
            start_server(host, port, &db_url).await?;
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
            run_sync(platform, all, &config).await?;
        }
        Commands::Config { get, set, list, init } => {
            let config_path = cli.config.clone()
                .unwrap_or_else(|| PathBuf::from("obsidian-bounty-finder.yaml"));
            run_config(get, set, list, init, &config_path).await?;
        }
        Commands::Status => {
            show_status().await?;
        }
    }

    Ok(())
}

async fn get_database(url: &str) -> Result<Database, Box<dyn std::error::Error>> {
    let db = Database::new(url).await?;
    db.init().await?;
    Ok(db)
}

async fn run_scan(
    platform: Option<String>,
    all: bool,
    _force: bool,
    _use_rendering: bool,
    _take_screenshots: bool,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = get_database(&config.database.url).await?;

    let mut registry = AdapterRegistry::new();

    let github_token = config.platforms.github_token.clone();
    let gitcoin_key = config.platforms.gitcoin_api_key.clone();
    let github_org = config.platforms.github_organization.clone();
    let hackerone_key = config.platforms.hackerone_api_key.clone();
    let hackerone_user = config.platforms.hackerone_username.clone();
    let bugcrowd_key = config.platforms.bugcrowd_api_key.clone();
    let bugcrowd_user = config.platforms.bugcrowd_username.clone();
    let laborx_key = config.platforms.laborx_api_key.clone();
    let dework_key = config.platforms.dework_api_key.clone();

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
    platform: Option<String>,
    status: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "sqlite://data/bounties.db".to_string()
    });
    let db = get_database(&database_url).await?;

    let platform_filter = platform.map(|p| obsidian_domain::Platform::parse(&p));
    let status_filter = status.map(|s| obsidian_domain::BountyStatus::parse(&s));

    let bounties = db
        .list_bounties_filtered(
            platform_filter.as_ref(),
            status_filter.as_ref(),
            limit,
            offset,
        )
        .await?;

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
    host: String,
    port: u16,
    database_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting API server on {}:{}", host, port);
    println!("Database: {}", database_url);
    println!("API server functionality coming soon!");
    Ok(())
}

async fn configure_notify(_channel: String, _test: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Notification configuration coming soon!");
    Ok(())
}

async fn run_sync(platform: Option<String>, all: bool, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let sync_all = all || platform.is_none();
    run_scan(platform, sync_all, true, config.scraper.use_rendering, config.scraper.take_screenshots, config).await
}

async fn run_config(
    get: Option<String>,
    set: Option<(String, String)>,
    list: bool,
    init: bool,
    config_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if init {
        let config = Config::default();
        config.save(&config_path.to_path_buf())?;
        println!("Created config file at {}", config_path.display());
        return Ok(());
    }
    
    let mut config = Config::load(&Some(config_path.to_path_buf()));
    
    if list {
        println!("Current configuration:");
        println!("{}", serde_yaml::to_string(&config)?);
    }
    
    if let Some((key, value)) = set {
        match key.as_str() {
            "database.url" => config.database.url = value.clone(),
            "scraper.use_rendering" => config.scraper.use_rendering = value.parse().unwrap_or(true),
            "scraper.take_screenshots" => config.scraper.take_screenshots = value.parse().unwrap_or(false),
            "scraper.timeout_secs" => config.scraper.timeout_secs = value.parse().unwrap_or(30),
            _ => {
                eprintln!("Unknown config key: {}. Use 'config --list' to see available keys.", key);
                return Ok(());
            }
        }
        config.save(&config_path.to_path_buf())?;
        println!("Updated {} = {}", key, value);
    }
    
    if let Some(key) = get {
        let value = match key.as_str() {
            "database.url" => config.database.url,
            "scraper.use_rendering" => config.scraper.use_rendering.to_string(),
            "scraper.take_screenshots" => config.scraper.take_screenshots.to_string(),
            "scraper.timeout_secs" => config.scraper.timeout_secs.to_string(),
            _ => {
                eprintln!("Unknown config key: {}", key);
                return Ok(());
            }
        };
        println!("{} = {}", key, value);
    }
    
    Ok(())
}

async fn show_status() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "sqlite://data/bounties.db".to_string()
    });
    let db = get_database(&database_url).await?;

    println!("\n=== ObsidianBountyFinder Status ===\n");

    let total = db.count_bounties().await?;
    let bounties = db.list_bounties(1000, 0).await?;
    let active = bounties
        .iter()
        .filter(|b| b.status == obsidian_domain::BountyStatus::Active)
        .count();

    println!("Database:");
    println!("  URL: {}", database_url);
    println!("  Total bounties: {}", total);
    println!("  Active bounties: {}", active);

    println!("\nUse 'obsidian-bounty-finder config --list' to see platform configuration.");

    Ok(())
}
