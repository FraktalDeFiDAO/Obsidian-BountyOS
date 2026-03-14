use crate::{AdapterError, AdapterResult, BountyAdapter, SearchQuery};
use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const DEWORK_API_URL: &str = "https://api.dework.com/v1";

pub struct DeWorkAdapter {
    client: Client,
    api_key: Option<String>,
}

impl DeWorkAdapter {
    pub fn new(api_key: Option<String>) -> Self {
        let client = Client::builder()
            .user_agent("ObsidianBountyFinder")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, api_key }
    }

    pub async fn fetch_tasks(&self) -> AdapterResult<Vec<DeWorkTask>> {
        let url = format!("{}/tasks", DEWORK_API_URL);
        
        let mut request = self.client.get(&url);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            AdapterError::Network(e.to_string())
        })?;

        if response.status() == 401 {
            return Err(AdapterError::Auth("Invalid DeWork credentials".to_string()));
        }

        if response.status() == 404 {
            return Ok(Vec::new());
        }

        if !response.status().is_success() {
            return Err(AdapterError::Api(format!("DeWork API error: {}", response.status())));
        }

        let tasks: DeWorkResponse = response.json().await.map_err(|e| {
            AdapterError::Parse(e.to_string())
        })?;

        Ok(tasks.items)
    }

    fn task_to_bounty(&self, task: &DeWorkTask) -> Bounty {
        let mut bounty = Bounty::new(
            task.id.to_string(),
            Platform::DeWork,
            task.title.clone(),
            task.url.clone().unwrap_or_default(),
            BountyType::Task,
        );

        bounty.description = task.description.clone().unwrap_or_default();
        
        if let Some(price) = task.price {
            bounty.reward_min = rust_decimal::Decimal::try_from(price).ok();
            bounty.reward_max = bounty.reward_min;
            bounty.reward_currency = Some("USD".to_string());
        }

        if let Some(skills) = &task.skills {
            bounty.skills = skills.clone();
        }

        if let Some(status) = &task.status {
            bounty.status = match status.as_str() {
                "open" | "active" => BountyStatus::Active,
                "closed" | "completed" => BountyStatus::Closed,
                "draft" => BountyStatus::Draft,
                _ => BountyStatus::Active,
            };
        }

        bounty.metadata = serde_json::json!({
            "task_id": task.id,
            "title": task.title,
            "status": task.status,
            "price": task.price,
            "currency": task.currency,
            "assignee": task.assignee,
            "due_date": task.due_date,
            "project_name": task.project_name,
        });

        bounty
    }
}

#[async_trait]
impl BountyAdapter for DeWorkAdapter {
    fn platform(&self) -> Platform {
        Platform::DeWork
    }

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>> {
        let tasks = self.fetch_tasks().await?;

        let bounties: Vec<Bounty> = tasks
            .into_iter()
            .filter(|t| t.status.as_deref() == Some("open") || t.status.as_deref() == Some("active"))
            .map(|t| self.task_to_bounty(&t))
            .collect();

        Ok(bounties)
    }

    fn supports_hooks(&self) -> bool {
        false
    }
}

#[derive(Debug, Deserialize)]
struct DeWorkResponse {
    items: Vec<DeWorkTask>,
    total: i32,
}

#[derive(Debug, Deserialize)]
struct DeWorkTask {
    id: String,
    title: String,
    description: Option<String>,
    url: Option<String>,
    status: Option<String>,
    price: Option<f64>,
    currency: Option<String>,
    skills: Option<Vec<String>>,
    assignee: Option<String>,
    due_date: Option<String>,
    project_name: Option<String>,
}
