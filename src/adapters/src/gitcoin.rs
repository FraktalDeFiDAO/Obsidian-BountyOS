use crate::{AdapterError, AdapterResult, BountyAdapter, SearchQuery};
use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use reqwest::Client;
use serde::Deserialize;

const GITCOIN_API_URL: &str = "https://gitcoin.co/grants/v1";

pub struct GitcoinAdapter {
    client: Client,
    api_key: Option<String>,
}

impl GitcoinAdapter {
    pub fn new(api_key: Option<String>) -> Self {
        let client = Client::builder()
            .user_agent("ObsidianBountyFinder")
            .build()
            .expect("Failed to create HTTP client");

        Self { client, api_key }
    }

    pub async fn fetch_grants(&self) -> AdapterResult<Vec<GitcoinGrant>> {
        let url = format!("{}/grants.json", GITCOIN_API_URL);

        let mut request = self.client.get(&url);

        if let Some(key) = &self.api_key {
            request = request.header("X-Api-Key", key);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AdapterError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AdapterError::Api(format!(
                "Gitcoin API error: {}",
                response.status()
            )));
        }

        let grants: Vec<GitcoinGrant> = response
            .json()
            .await
            .map_err(|e| AdapterError::Parse(e.to_string()))?;

        Ok(grants)
    }

    fn grant_to_bounty(&self, grant: &GitcoinGrant) -> Bounty {
        let mut bounty = Bounty::new(
            grant.id.to_string(),
            Platform::Gitcoin,
            grant.title.clone(),
            grant.url.clone(),
            BountyType::Grant,
        );

        bounty.description = grant.description.clone().unwrap_or_default();
        bounty.tags = grant.tags.clone();

        if grant.amount_received > 0.0 {
            bounty.reward_min = rust_decimal::Decimal::try_from(grant.amount_received).ok();
            bounty.reward_max = bounty.reward_min;
        }

        if let Some(token) = &grant.token {
            bounty.reward_currency = Some(token.to_uppercase());
        }

        bounty.metadata = serde_json::json!({
            "grant_id": grant.id,
            "amount_received": grant.amount_received,
            "amount_goal": grant.amount_goal,
            "token": grant.token,
            "logo_url": grant.logo_url,
            "owner": grant.owner,
            "active": grant.active,
            "visibility": grant.visibility,
        });

        bounty
    }
}

#[async_trait]
impl BountyAdapter for GitcoinAdapter {
    fn platform(&self) -> Platform {
        Platform::Gitcoin
    }

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>> {
        let grants = self.fetch_grants().await?;

        let bounties: Vec<Bounty> = grants
            .into_iter()
            .filter(|g| g.active)
            .map(|g| self.grant_to_bounty(&g))
            .collect();

        Ok(bounties)
    }

    fn validate_config(&self) -> AdapterResult<()> {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct GitcoinGrant {
    id: i64,
    title: String,
    #[serde(default)]
    description: Option<String>,
    url: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(rename = "amount_received", default)]
    amount_received: f64,
    #[serde(rename = "amount_goal", default)]
    amount_goal: f64,
    #[serde(default)]
    token: Option<String>,
    #[serde(rename = "logo_url", default)]
    logo_url: Option<String>,
    #[serde(default)]
    owner: Option<String>,
    #[serde(default)]
    active: bool,
    #[serde(default)]
    visibility: Option<String>,
    #[serde(default)]
    created_at: Option<String>,
}
