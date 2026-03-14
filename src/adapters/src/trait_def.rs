use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyType, Platform};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: Option<String>,
    pub bounty_type: Option<BountyType>,
    pub status: Option<String>,
    pub min_reward: Option<f64>,
    pub max_reward: Option<f64>,
    pub skills: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: None,
            bounty_type: None,
            status: None,
            min_reward: None,
            max_reward: None,
            skills: None,
            tags: None,
            limit: Some(50),
            offset: Some(0),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Rate limited, retry after: {0}")]
    RateLimited(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

pub type AdapterResult<T> = Result<T, AdapterError>;

#[async_trait]
pub trait BountyAdapter: Send + Sync {
    fn platform(&self) -> Platform;

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>>;

    async fn fetch_updates(
        &self,
        _since: chrono::DateTime<chrono::Utc>,
    ) -> AdapterResult<Vec<Bounty>> {
        Err(AdapterError::NotImplemented(
            "fetch_updates not implemented for this platform".to_string(),
        ))
    }

    async fn fetch_bounty(&self, _external_id: &str) -> AdapterResult<Option<Bounty>> {
        Err(AdapterError::NotImplemented(
            "fetch_bounty not implemented for this platform".to_string(),
        ))
    }

    async fn search(&self, _query: &SearchQuery) -> AdapterResult<Vec<Bounty>> {
        Err(AdapterError::NotImplemented(
            "search not implemented for this platform".to_string(),
        ))
    }

    fn supports_hooks(&self) -> bool {
        false
    }

    fn hook_url(&self) -> Option<String> {
        None
    }

    fn validate_config(&self) -> AdapterResult<()> {
        Ok(())
    }
}

pub struct AdapterRegistry {
    adapters: HashMap<Platform, Box<dyn BountyAdapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    pub fn register(&mut self, adapter: Box<dyn BountyAdapter>) {
        let platform = adapter.platform();
        self.adapters.insert(platform, adapter);
    }

    pub fn get(&self, platform: &Platform) -> Option<&dyn BountyAdapter> {
        self.adapters.get(platform).map(|b| b.as_ref())
    }

    pub fn platforms(&self) -> Vec<Platform> {
        self.adapters.keys().cloned().collect()
    }

    pub fn adapter_count(&self) -> usize {
        self.adapters.len()
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}
