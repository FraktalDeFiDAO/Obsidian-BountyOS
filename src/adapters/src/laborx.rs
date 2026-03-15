use crate::{AdapterError, AdapterResult, BountyAdapter};
use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const LABORX_API_URL: &str = "https://api.laborx.com/v1";

pub struct LaborXAdapter {
    client: Client,
    api_key: Option<String>,
}

impl LaborXAdapter {
    pub fn new(api_key: Option<String>) -> Self {
        let client = Client::builder()
            .user_agent("ObsidianBountyFinder")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, api_key }
    }

    pub async fn fetch_jobs(&self) -> AdapterResult<Vec<LaborXJob>> {
        if self.api_key.is_none() {
            return Ok(Vec::new());
        }

        let url = format!("{}/jobs", LABORX_API_URL);

        let mut request = self.client.get(&url);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request
            .send()
            .await
            .map_err(|e| AdapterError::Network(e.to_string()))?;

        if response.status() == 401 {
            return Err(AdapterError::Auth("Invalid LaborX credentials".to_string()));
        }

        if response.status() == 404 {
            return Ok(Vec::new());
        }

        if !response.status().is_success() {
            return Err(AdapterError::Api(format!(
                "LaborX API error: {}",
                response.status()
            )));
        }

        let jobs: LaborXResponse = response
            .json()
            .await
            .map_err(|e| AdapterError::Parse(e.to_string()))?;

        Ok(jobs.items)
    }

    fn job_to_bounty(&self, job: &LaborXJob) -> Bounty {
        let mut bounty = Bounty::new(
            job.id.to_string(),
            Platform::LaborX,
            job.title.clone(),
            job.url.clone().unwrap_or_default(),
            BountyType::Task,
        );

        bounty.description = job.description.clone().unwrap_or_default();

        if let Some(payment) = &job.payment {
            if let Some(amount) = payment.amount {
                bounty.reward_min = rust_decimal::Decimal::try_from(amount).ok();
                bounty.reward_max = bounty.reward_min;
            }
            if let Some(currency) = &payment.currency {
                bounty.reward_currency = Some(currency.clone());
            }
        }

        if let Some(skills) = &job.required_skills {
            bounty.skills = skills.clone();
        }

        if let Some(cat) = &job.category {
            bounty.tags.push(cat.clone());
        }

        bounty.status = match job.status.as_deref() {
            Some("active") => BountyStatus::Active,
            Some("closed") => BountyStatus::Closed,
            _ => BountyStatus::Active,
        };

        bounty.metadata = serde_json::json!({
            "job_id": job.id,
            "title": job.title,
            "status": job.status,
            "employment_type": job.employment_type,
            "category": job.category,
            "experience_level": job.experience_level,
        });

        bounty
    }
}

#[async_trait]
impl BountyAdapter for LaborXAdapter {
    fn platform(&self) -> Platform {
        Platform::LaborX
    }

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>> {
        let jobs = self.fetch_jobs().await?;

        let bounties: Vec<Bounty> = jobs
            .into_iter()
            .filter(|j| j.status.as_deref() == Some("active"))
            .map(|j| self.job_to_bounty(&j))
            .collect();

        Ok(bounties)
    }

    fn supports_hooks(&self) -> bool {
        false
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LaborXResponse {
    items: Vec<LaborXJob>,
    total: i32,
    page: i32,
}

#[derive(Debug, Deserialize)]
pub struct LaborXJob {
    id: String,
    title: String,
    description: Option<String>,
    url: Option<String>,
    status: Option<String>,
    employment_type: Option<String>,
    category: Option<String>,
    required_skills: Option<Vec<String>>,
    experience_level: Option<String>,
    payment: Option<LaborXPayment>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LaborXPayment {
    amount: Option<f64>,
    currency: Option<String>,
    payment_type: Option<String>,
}
