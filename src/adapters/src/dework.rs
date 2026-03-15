use crate::{AdapterError, AdapterResult, BountyAdapter};
use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use reqwest::Client;
use std::time::Duration;

pub struct DeWorkAdapter {
    client: Client,
}

impl DeWorkAdapter {
    pub fn new(_api_key: Option<String>) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn fetch_bounties(&self) -> AdapterResult<Vec<Bounty>> {
        let url = "https://app.dework.xyz/bounties";
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| AdapterError::Network(e.to_string()))?;

        let html = response.text().await
            .map_err(|e| AdapterError::Parse(e.to_string()))?;

        let bounties = self.parse_bounties_from_html(&html)?;
        Ok(bounties)
    }

    fn parse_bounties_from_html(&self, html: &str) -> AdapterResult<Vec<Bounty>> {
        let mut bounties = Vec::new();

        if let Some(start) = html.find("__NEXT_DATA__\" type=\"application/json\">") {
            if let Some(end) = html[start..].find("</script>") {
                let json_str = &html[start + 36..start + end];
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(tasks) = data.get("props")
                        .and_then(|p| p.get("apolloState"))
                        .and_then(|a| a.get("data"))
                        .and_then(|d| d.get("ROOT_QUERY"))
                        .and_then(|r| r.get("getTasks"))
                        .and_then(|t| t.as_array())
                    {
                        for task in tasks {
                            if let Some(bounty) = self.extract_bounty_from_task(task) {
                                bounties.push(bounty);
                            }
                        }
                    }
                }
            }
        }

        if bounties.is_empty() {
            return Ok(Vec::new());
        }

        Ok(bounties)
    }

    fn extract_bounty_from_task(&self, task: &serde_json::Value) -> Option<Bounty> {
        let id = task.get("id")?.as_str()?;
        let title = task.get("title").or_else(|| task.get("name"))?.as_str()?;
        let url = task.get("url").or_else(|| task.get("link"))?.as_str()?;
        let status = task.get("status").or_else(|| task.get("state"))?.as_str()?;
        let reward = task.get("reward").or_else(|| task.get("price")).or_else(|| task.get("paymentAmount"));
        let description = task.get("description").or_else(|| task.get("body"));

        let mut bounty = Bounty::new(
            id.to_string(),
            Platform::DeWork,
            title.to_string(),
            url.to_string(),
            BountyType::Task,
        );

        bounty.status = match status {
            "open" | "active" | "OPEN" | "ACTIVE" => BountyStatus::Active,
            "closed" | "completed" | "CLOSED" | "COMPLETED" => BountyStatus::Closed,
            "draft" | "DRAFT" => BountyStatus::Draft,
            _ => BountyStatus::Active,
        };

        if let Some(r) = reward {
            if let Some(amount) = r.as_f64() {
                bounty.reward_min = rust_decimal::Decimal::try_from(amount).ok();
                bounty.reward_max = bounty.reward_min;
                bounty.reward_currency = Some("USD".to_string());
            } else if let Some(s) = r.as_str() {
                if let Ok(amount) = s.parse::<f64>() {
                    bounty.reward_min = rust_decimal::Decimal::try_from(amount).ok();
                    bounty.reward_max = bounty.reward_min;
                    bounty.reward_currency = Some("USD".to_string());
                }
            }
        }

        if let Some(desc) = description {
            if let Some(s) = desc.as_str() {
                bounty.description = s.to_string();
            }
        }

        if let Some(tags) = task.get("skills").or_else(|| task.get("tagNames")).or_else(|| task.get("tags")) {
            if let Some(arr) = tags.as_array() {
                bounty.tags = arr.iter()
                    .filter_map(|t| t.as_str().map(String::from))
                    .collect();
            }
        }

        Some(bounty)
    }
}

#[async_trait]
impl BountyAdapter for DeWorkAdapter {
    fn platform(&self) -> Platform {
        Platform::DeWork
    }

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>> {
        self.fetch_bounties().await
    }

    fn supports_hooks(&self) -> bool {
        false
    }
}
