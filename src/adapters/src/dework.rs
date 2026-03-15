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
                    {
                        if let Some(obj) = tasks.as_object() {
                            for (key, value) in obj.iter() {
                                if key.contains("Task") || key.contains("Bounty") {
                                    if let Some(task_arr) = value.as_array() {
                                        for task in task_arr {
                                            if let Some(bounty) = self.extract_bounty_from_task(task) {
                                                bounties.push(bounty);
                                            }
                                        }
                                    } else if let Some(obj_val) = value.as_object() {
                                        if let Some(bounty) = self.extract_bounty_from_task(&serde_json::json!([obj_val])) {
                                            bounties.push(bounty);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    if bounties.is_empty() {
                        if let Some(page_props) = data.get("props")
                            .and_then(|p| p.get("pageProps"))
                        {
                            if let Some(obj) = page_props.as_object() {
                                for (key, value) in obj.iter() {
                                    if key.to_lowercase().contains("task") || key.to_lowercase().contains("bounty") {
                                        if let Some(arr) = value.as_array() {
                                            for task in arr {
                                                if let Some(bounty) = self.extract_bounty_from_task(task) {
                                                    bounties.push(bounty);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if bounties.is_empty() {
            tracing::debug!("No bounties found in DeWork HTML");
        }

        Ok(bounties)
    }

    fn extract_bounty_from_task(&self, task: &serde_json::Value) -> Option<Bounty> {
        let task = if let Some(arr) = task.as_array() {
            arr.first()?
        } else {
            task
        };

        let id = task.get("id")
            .or_else(|| task.get("taskId"))
            .or_else(|| task.get("uuid"))
            .and_then(|v| v.as_str())?;
            
        let title = task.get("title")
            .or_else(|| task.get("name"))
            .or_else(|| task.get("heading"))
            .and_then(|v| v.as_str())?;
            
        let url = task.get("url")
            .or_else(|| task.get("link"))
            .or_else(|| task.get("permalink"))
            .or_else(|| task.get("externalUrl"))
            .and_then(|v| v.as_str())
            .map(|u| {
                if u.starts_with("http") {
                    u.to_string()
                } else {
                    format!("https://app.dework.xyz{}", u)
                }
            })
            .unwrap_or_else(|| "https://app.dework.xyz/bounties".to_string());
            
        let status = task.get("status")
            .or_else(|| task.get("state"))
            .or_else(|| task.get("taskStatus"))
            .and_then(|v| v.as_str())
            .unwrap_or("open");

        let reward = task.get("reward")
            .or_else(|| task.get("price"))
            .or_else(|| task.get("paymentAmount"))
            .or_else(|| task.get("payment"))
            .or_else(|| task.get("value"));
            
        let description = task.get("description")
            .or_else(|| task.get("body"))
            .or_else(|| task.get("summary"));

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
