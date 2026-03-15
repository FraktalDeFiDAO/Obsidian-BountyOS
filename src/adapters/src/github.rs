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
        if self.token.is_none() {
            return Ok(Vec::new());
        }

        let search_queries = [
            "label:bounty is:issue state:open",
            "label:reward is:issue state:open", 
            "label:\"help wanted\" is:issue state:open",
            "label:paid is:issue state:open",
        ];

        let mut all_issues: Vec<GitHubIssue> = Vec::new();
        let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

        for query in &search_queries {
            let url = format!("{}/search/issues", GITHUB_API_URL);
            let params = vec![
                ("q".to_string(), query.to_string()),
                ("per_page".to_string(), "100".to_string()),
                ("page".to_string(), page.to_string()),
                ("sort".to_string(), "created".to_string()),
                ("order".to_string(), "desc".to_string()),
            ];

            let mut request = self.client.get(&url).query(&params);

            if let Some((key, value)) = self.build_auth_header() {
                request = request.header(&key, &value);
            }

            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => {
                    tracing::warn!("GitHub search request failed: {}", e);
                    continue;
                }
            };

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
                tracing::warn!("GitHub search API error: {}", response.status());
                continue;
            }

            #[derive(Deserialize)]
            struct SearchResult {
                items: Vec<GitHubIssue>,
            }

            if let Ok(result) = response.json::<SearchResult>().await {
                for issue in result.items {
                    if seen_ids.insert(issue.id.to_string()) {
                        all_issues.push(issue);
                    }
                }
            }
        }

        Ok(all_issues)
    }

    pub async fn fetch_with_gh_cli(&self) -> AdapterResult<Vec<GitHubIssue>> {
        let queries = [
            "label:bounty is:issue state:open",
            "label:reward is:issue state:open",
        ];

        let mut all_issues: Vec<GitHubIssue> = Vec::new();

        for query in &queries {
            let output = std::process::Command::new("gh")
                .args(&["search", "issues", query, "--limit", "100", "--json",
                        "id,number,title,body,url,labels,state,createdAt,user"])
                .output();

            match output {
                Ok(out) if out.status.success() => {
                    if let Ok(items) = serde_json::from_str::<Vec<serde_json::Value>>(&String::from_utf8_lossy(&out.stdout)) {
                        for item in items {
                            let labels: Vec<GitHubLabel> = item.get("labels")
                                .and_then(|v| v.as_array())
                                .map(|arr| arr.iter().filter_map(|l| {
                                    Some(GitHubLabel {
                                        name: l.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string(),
                                        color: l.get("color").and_then(|c| c.as_str()).unwrap_or("").to_string(),
                                    })
                                }).collect())
                                .unwrap_or_default();
                            
                            let issue = GitHubIssue {
                                id: item.get("id").and_then(|v| v.as_i64()).unwrap_or(0),
                                number: item.get("number").and_then(|v| v.as_i64()).unwrap_or(0),
                                title: item.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                body: item.get("body").and_then(|v| v.as_str()).map(String::from),
                                state: item.get("state").and_then(|v| v.as_str()).unwrap_or("open").to_string(),
                                html_url: item.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                labels,
                                comments: 0,
                                pull_request: None,
                                created_at: item.get("createdAt").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                updated_at: "".to_string(),
                            };
                            all_issues.push(issue);
                        }
                    }
                }
                Ok(out) => {
                    tracing::warn!("gh CLI failed: {}", String::from_utf8_lossy(&out.stderr));
                }
                Err(e) => {
                    tracing::warn!("gh CLI not available: {}", e);
                }
            }
        }

        Ok(all_issues)
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
        
        let issues_result = self.fetch_issues(1).await;
        
        let issues = match issues_result {
            Ok(i) if !i.is_empty() => i,
            Ok(_) => {
                tracing::info!("API returned empty, trying gh CLI fallback...");
                match self.fetch_with_gh_cli().await {
                    Ok(gh_issues) => gh_issues,
                    Err(e) => {
                        tracing::warn!("gh CLI fallback failed: {:?}", e);
                        Vec::new()
                    }
                }
            }
            Err(AdapterError::RateLimited(_)) => {
                tracing::warn!("Rate limited, trying gh CLI fallback...");
                match self.fetch_with_gh_cli().await {
                    Ok(gh_issues) => gh_issues,
                    Err(e) => {
                        tracing::warn!("gh CLI fallback failed: {:?}", e);
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                tracing::warn!("GitHub API error, trying gh CLI fallback: {:?}", e);
                match self.fetch_with_gh_cli().await {
                    Ok(gh_issues) => gh_issues,
                    Err(e) => {
                        tracing::warn!("gh CLI fallback failed: {:?}", e);
                        Vec::new()
                    }
                }
            }
        };

        for issue in issues {
            if let Some(bounty) = self.issue_to_bounty(&issue) {
                all_bounties.push(bounty);
            }
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
