use crate::{AdapterError, AdapterResult, BountyAdapter};
use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use reqwest::Client;
use serde::Deserialize;

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GitHubAdapter {
    client: Client,
    token: Option<String>,
    organization: Option<String>,
}

impl GitHubAdapter {
    pub fn new(token: Option<String>, organization: Option<String>) -> Self {
        let client = Client::builder()
            .user_agent("ObsidianBountyFinder")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            token,
            organization,
        }
    }

    pub fn with_url(token: Option<String>, organization: Option<String>, _base_url: &str) -> Self {
        Self::new(token, organization)
    }

    fn build_auth_header(&self) -> Option<(String, String)> {
        self.token
            .as_ref()
            .map(|t| ("Authorization".to_string(), format!("Bearer {}", t)))
    }

    pub async fn fetch_issues(&self, page: u32) -> AdapterResult<Vec<GitHubIssue>> {
        let mut url = format!("{}/issues", GITHUB_API_URL);

        let params = vec![
            ("state".to_string(), "open".to_string()),
            ("per_page".to_string(), "100".to_string()),
            ("page".to_string(), page.to_string()),
            ("sort".to_string(), "created".to_string()),
            ("direction".to_string(), "desc".to_string()),
        ];

        if let Some(org) = &self.organization {
            url = format!("{}/orgs/{}/issues", GITHUB_API_URL, org);
        }

        let mut request = self.client.get(&url).query(&params);

        if let Some((key, value)) = self.build_auth_header() {
            request = request.header(&key, &value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AdapterError::Network(e.to_string()))?;

        if response.status() == 401 {
            return Err(AdapterError::Auth("Invalid GitHub token".to_string()));
        }

        if response.status() == 403 {
            let remaining = response
                .headers()
                .get("X-RateLimit-Remaining")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("0");

            if remaining == "0" {
                return Err(AdapterError::RateLimited("Rate limit exceeded".to_string()));
            }
        }

        if !response.status().is_success() {
            return Err(AdapterError::Api(format!(
                "GitHub API error: {}",
                response.status()
            )));
        }

        let issues: Vec<GitHubIssue> = response
            .json()
            .await
            .map_err(|e| AdapterError::Parse(e.to_string()))?;

        Ok(issues)
    }

    fn issue_to_bounty(&self, issue: &GitHubIssue) -> Option<Bounty> {
        if issue.pull_request.is_some() {
            return None;
        }

        let has_bounty_label = issue.labels.iter().any(|l| {
            let name = l.name.to_lowercase();
            name.contains("bounty")
                || name.contains("reward")
                || name.contains("paid")
                || name.contains("help wanted")
                || name.contains("funding")
        });

        if !has_bounty_label {
            return None;
        }

        let (reward_min, reward_max, currency) = Self::extract_reward(&issue.body);
        let tags: Vec<String> = issue.labels.iter().map(|l| l.name.clone()).collect();
        let skills = Self::extract_skills(&issue.body);

        let bounty = Bounty::new(
            issue.number.to_string(),
            Platform::GitHub,
            issue.title.clone(),
            issue.html_url.clone(),
            BountyType::Bounty,
        );

        Some(bounty).map(|b| {
            let mut b = b;
            b.description = issue.body.clone().unwrap_or_default();
            b.status = if issue.state == "open" {
                BountyStatus::Active
            } else {
                BountyStatus::Closed
            };
            b.tags = tags;
            b.skills = skills;
            b.reward_min = reward_min;
            b.reward_max = reward_max;
            b.reward_currency = currency;
            b.metadata = serde_json::json!({
                "comments": issue.comments,
                "labels": issue.labels.iter().map(|l| &l.name).collect::<Vec<_>>(),
            });
            b
        })
    }

    fn extract_reward(
        body: &Option<String>,
    ) -> (
        Option<rust_decimal::Decimal>,
        Option<rust_decimal::Decimal>,
        Option<String>,
    ) {
        let body = match body {
            Some(b) => b,
            None => return (None, None, None),
        };

        let usd_patterns = [
            r"(?i)\$([0-9,]+(?:\.[0-9]+)?)\s*(?:USD|usd)",
            r"(?i)(?:reward|bounty|paid)[:\s]*\$?([0-9,]+(?:\.[0-9]+)?)\s*(?:USD|usd)",
            r"(?i)(?:up to|maximum)[:\s]*\$?([0-9,]+(?:\.[0-9]+)?)\s*(?:USD|usd)",
        ];

        let mut min_reward: Option<rust_decimal::Decimal> = None;
        let mut max_reward: Option<rust_decimal::Decimal> = None;
        let mut currency = Some("USD".to_string());

        for pattern in &usd_patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(caps) = re.captures(body) {
                    if let Some(m) = caps.get(1) {
                        let amount: f64 = m.as_str().replace(',', "").parse().unwrap_or(0.0);
                        if amount > 0.0 {
                            let dec = rust_decimal::Decimal::try_from(amount).ok();
                            if min_reward.is_none() {
                                min_reward = dec;
                                max_reward = dec;
                            } else if let Some(d) = dec {
                                if let Some(max) = max_reward {
                                    if d > max {
                                        max_reward = Some(d);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if min_reward.is_none() {
            currency = None;
        }

        (min_reward, max_reward, currency)
    }

    fn extract_skills(body: &Option<String>) -> Vec<String> {
        let body = match body {
            Some(b) => b,
            None => return Vec::new(),
        };

        let skill_keywords = [
            "rust",
            "javascript",
            "typescript",
            "python",
            "go",
            "java",
            "c++",
            "c#",
            "react",
            "vue",
            "angular",
            "node",
            "docker",
            "kubernetes",
            "aws",
            "gcp",
            "solidity",
            "rust",
            "graphql",
            "rest",
            "sql",
            "postgresql",
            "mongodb",
        ];

        let body_lower = body.to_lowercase();

        skill_keywords
            .iter()
            .filter(|s| body_lower.contains(*s))
            .map(|s| s.to_string())
            .collect()
    }
}

#[async_trait]
impl BountyAdapter for GitHubAdapter {
    fn platform(&self) -> Platform {
        Platform::GitHub
    }

    async fn fetch_all(&self) -> AdapterResult<Vec<Bounty>> {
        let mut all_bounties = Vec::new();
        let mut page = 1;
        let max_pages = 10;

        while page <= max_pages {
            let issues = match self.fetch_issues(page).await {
                Ok(i) => i,
                Err(AdapterError::RateLimited(_)) => break,
                Err(e) => return Err(e),
            };

            if issues.is_empty() {
                break;
            }

            for issue in issues {
                if let Some(bounty) = self.issue_to_bounty(&issue) {
                    all_bounties.push(bounty);
                }
            }

            page += 1;
        }

        Ok(all_bounties)
    }

    fn validate_config(&self) -> AdapterResult<()> {
        if let Some(token) = &self.token {
            if token.is_empty() {
                return Err(AdapterError::Config("GitHub token is empty".to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GitHubIssue {
    id: i64,
    number: i64,
    title: String,
    body: Option<String>,
    state: String,
    html_url: String,
    #[serde(default)]
    labels: Vec<GitHubLabel>,
    comments: i32,
    #[serde(default)]
    pull_request: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitHubLabel {
    name: String,
    color: String,
}
