use crate::{AdapterError, AdapterResult, BountyAdapter};
use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyType, Platform};
use reqwest::Client;
use serde::Deserialize;

const HACKERONE_API_URL: &str = "https://api.hackerone.com/api/v1";

pub struct HackerOneAdapter {
    client: Client,
    api_key: Option<String>,
    username: Option<String>,
}

impl HackerOneAdapter {
    pub fn new(api_key: Option<String>, username: Option<String>) -> Self {
        let client = Client::builder()
            .user_agent("ObsidianBountyFinder")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key,
            username,
        }
    }

    pub async fn fetch_programs(&self) -> AdapterResult<Vec<HackerOneProgram>> {
        if self.api_key.is_none() || self.username.is_none() {
            return Ok(Vec::new());
        }

        let url = format!("{}/programs", HACKERONE_API_URL);

        let mut request = self.client.get(&url);

        if let (Some(key), Some(user)) = (&self.api_key, &self.username) {
            request = request.basic_auth(user, Some(key));
        }

        let response = request
            .send()
            .await
            .map_err(|e| AdapterError::Network(e.to_string()))?;

        if response.status() == 401 {
            return Err(AdapterError::Auth(
                "Invalid HackerOne credentials".to_string(),
            ));
        }

        if !response.status().is_success() {
            return Err(AdapterError::Api(format!(
                "HackerOne API error: {}",
                response.status()
            )));
        }

        let programs: HackerOneResponse = response
            .json()
            .await
            .map_err(|e| AdapterError::Parse(e.to_string()))?;

        Ok(programs.data)
    }

    fn program_to_bounty(&self, program: &HackerOneProgram) -> Bounty {
        let mut bounty = Bounty::new(
            program.id.to_string(),
            Platform::HackerOne,
            program.attributes.name.clone(),
            format!("https://hackerone.com{}", program.attributes.handle),
            BountyType::BugBounty,
        );

        bounty.description = program.attributes.description.clone().unwrap_or_default();

        if let Some(eligibility) = &program.attributes.eligibility {
            bounty.tags = eligibility.iter().map(|s| s.as_str().to_string()).collect();
        }

        bounty.metadata = serde_json::json!({
            "program_id": program.id,
            "handle": program.attributes.handle,
            "state": program.attributes.state,
            "offering": program.attributes.offering,
            "currency": program.attributes.currency,
        });

        bounty
    }
}

#[async_trait]
impl BountyAdapter for HackerOneAdapter {
    fn platform(&self) -> Platform {
        Platform::HackerOne
    }

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>> {
        let programs = self.fetch_programs().await?;

        let bounties: Vec<Bounty> = programs
            .into_iter()
            .filter(|p| p.attributes.state == "active")
            .map(|p| self.program_to_bounty(&p))
            .collect();

        Ok(bounties)
    }

    fn validate_config(&self) -> AdapterResult<()> {
        if let Some(key) = &self.api_key {
            if key.is_empty() {
                return Err(AdapterError::Config(
                    "HackerOne API key is empty".to_string(),
                ));
            }
        }
        if let Some(user) = &self.username {
            if user.is_empty() {
                return Err(AdapterError::Config(
                    "HackerOne username is empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HackerOneResponse {
    data: Vec<HackerOneProgram>,
}

#[derive(Debug, Deserialize)]
pub struct HackerOneProgram {
    id: i64,
    #[serde(rename = "type")]
    _type: String,
    attributes: HackerOneAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HackerOneAttributes {
    name: String,
    handle: String,
    description: Option<String>,
    state: String,
    offering: Option<String>,
    currency: Option<String>,
    eligibility: Option<Vec<String>>,
}
