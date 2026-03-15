use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub database: DatabaseConfig,
    pub scraper: ScraperConfig,
    pub platforms: PlatformConfig,
    pub notifications: NotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://data/bounties.db".to_string(),
            pool_size: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperConfig {
    pub use_rendering: bool,
    pub take_screenshots: bool,
    pub timeout_secs: u32,
    pub user_agent: String,
}

impl Default for ScraperConfig {
    fn default() -> Self {
        Self {
            use_rendering: true,
            take_screenshots: false,
            timeout_secs: 30,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub github_token: Option<String>,
    pub github_organization: Option<String>,
    pub gitcoin_api_key: Option<String>,
    pub hackerone_api_key: Option<String>,
    pub hackerone_username: Option<String>,
    pub bugcrowd_api_key: Option<String>,
    pub bugcrowd_username: Option<String>,
    pub laborx_api_key: Option<String>,
    pub dework_api_key: Option<String>,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            github_token: std::env::var("GITHUB_TOKEN").ok(),
            github_organization: std::env::var("GITHUB_ORGANIZATION").ok(),
            gitcoin_api_key: std::env::var("GITCOIN_API_KEY").ok(),
            hackerone_api_key: std::env::var("HACKERONE_API_KEY").ok(),
            hackerone_username: std::env::var("HACKERONE_USERNAME").ok(),
            bugcrowd_api_key: std::env::var("BUGCROWD_API_KEY").ok(),
            bugcrowd_username: std::env::var("BUGCROWD_USERNAME").ok(),
            laborx_api_key: std::env::var("LABORX_API_KEY").ok(),
            dework_api_key: std::env::var("DEWORK_API_KEY").ok(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub telegram_bot_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub discord_webhook_url: Option<String>,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_from: Option<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            telegram_bot_token: std::env::var("TELEGRAM_BOT_TOKEN").ok(),
            telegram_chat_id: std::env::var("TELEGRAM_CHAT_ID").ok(),
            discord_webhook_url: std::env::var("DISCORD_WEBHOOK_URL").ok(),
            smtp_host: std::env::var("SMTP_HOST").ok(),
            smtp_port: std::env::var("SMTP_PORT").ok().and_then(|p| p.parse().ok()),
            smtp_username: std::env::var("SMTP_USERNAME").ok(),
            smtp_password: std::env::var("SMTP_PASSWORD").ok(),
            smtp_from: std::env::var("SMTP_FROM").ok(),
        }
    }
}

impl Config {
    pub fn load(path: &Option<PathBuf>) -> Self {
        let config_path = path
            .clone()
            .or_else(|| get_config_dir())
            .unwrap_or_else(|| PathBuf::from("obsidian-bounty-finder.yaml"));

        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(config) = serde_yaml::from_str(&content) {
                    info!("Loaded config from {}", config_path.display());
                    return config;
                }
            }
        }

        info!("Using default config");
        Self::default()
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

fn get_config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return std::env::var("APPDATA").ok().map(PathBuf::from);

    #[cfg(target_os = "macos")]
    return std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join("Library/Application Support"));

    #[cfg(target_os = "linux")]
    return std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|h| PathBuf::from(h).join(".config"))
        });

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return None;
}
