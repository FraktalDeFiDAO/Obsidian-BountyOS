use crate::{AdapterError, AdapterResult, BountyAdapter, SearchQuery};
use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use reqwest::Client;
use serde::Deserialize;

const BUGCROWD_API_URL: &str = "https://api.bugcrowd.com/v2";

pub struct BugcrowdAdapter {
    client: Client,
    api_key: Option<String>,
    username: Option<String>,
}

impl BugcrowdAdapter {
    pub fn new(api_key: Option<String>, username: Option<String>) -> Self {
        let client = Client::builder()
            .user_agent("ObsidianBountyFinder")
            .build()
            .expect("Failed to create HTTP client");

        Self { client, api_key, username }
    }

    pub async fn fetch_programs(&self) -> AdapterResult<Vec<BugcrowdProgram>> {
        let url = format!("{}/programs", BUGCROWD_API_URL);
        
        let mut request = self.client.get(&url);

        if let (Some(key), Some(user)) = (&self.api_key, &self.username) {
            request = request.basic_auth(user, Some(key));
        }

        let response = request.send().await.map_err(|e| {
            AdapterError::Network(e.to_string())
        })?;

        if response.status() == 401 {
            return Err(AdapterError::Auth("Invalid Bugcrowd credentials".to_string()));
        }

        if !response.status().is_success() {
            return Err(AdapterError::Api(format!("Bugcrowd API error: {}", response.status())));
        }

        let programs: BugcrowdResponse = response.json().await.map_err(|e| {
            AdapterError::Parse(e.to_string())
        })?;

        Ok(programs.programs)
    }

    fn program_to_bounty(&self, program: &BugcrowdProgram) -> Bounty {
        let mut bounty = Bounty::new(
            program.id.to_string(),
            Platform::Bugcrowd,
            program.name.clone(),
            program.url.clone().unwrap_or_default(),
            BountyType::BugBounty,
        );

        bounty.description = program.description.clone().unwrap_or_default();
        
        if let Some(status) = &program.status {
            bounty.status = match status.as_str() {
                "active" => BountyStatus::Active,
                "paused" => BountyStatus::Paused,
                _ => BountyStatus::Active,
            };
        }

        if let Some(t) = &program.bounty_type {
            bounty.tags.push(t.clone());
        }

        bounty.metadata = serde_json::json!({
            "program_id": program.id,
            "name": program.name,
            "status": program.status,
            "bounty_type": program.bounty_type,
            "allowed_categories": program.allowed_categories,
        });

        bounty
    }
}

#[async_trait]
impl BountyAdapter for BugcrowdAdapter {
    fn platform(&self) -> Platform {
        Platform::Bugcrowd
    }

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>> {
        let programs = self.fetch_programs().await?;

        let bounties: Vec<Bounty> = programs
            .into_iter()
            .filter(|p| p.status.as_deref() == Some("active"))
            .map(|p| self.program_to_bounty(&p))
            .collect();

        Ok(bounties)
    }

    fn validate_config(&self) -> AdapterResult<()> {
        if let Some(key) = &self.api_key {
            if key.is_empty() {
                return Err(AdapterError::Config("Bugcrowd API key is empty".to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct BugcrowdResponse {
    programs: Vec<BugcrowdProgram>,
}

#[derive(Debug, Deserialize)]
struct BugcrowdProgram {
    id: i64,
    name: String,
    description: Option<String>,
    url: Option<String>,
    status: Option<String>,
    bounty_type: Option<String>,
    allowed_categories: Option<Vec<String>>,
}
