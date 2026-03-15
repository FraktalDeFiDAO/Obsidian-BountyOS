use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedContent {
    pub html: String,
    pub text: Option<String>,
    pub json_data: Option<serde_json::Value>,
    pub screenshot: Option<Vec<u8>>,
    pub needs_rendering: bool,
    pub url: String,
}

pub struct WebScraper {
    client: Client,
    screenshot_mode: bool,
}

impl WebScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            screenshot_mode: false,
        }
    }

    pub fn with_screenshot_mode(mut self) -> Self {
        self.screenshot_mode = true;
        self
    }

    pub fn needs_rendering(url: &str) -> bool {
        let rendering_domains = [
            "app.dework.xyz",
            "app.oyster.xyz",
            "app.raidguild.xyz",
            "app.sophon.xyz",
            "galxe.com",
            "app.galxe.com",
            "layer3.xyz",
            "app.layer3.xyz",
            "rabbithole.gg",
            "app.rabbithole.gg",
        ];
        
        rendering_domains.iter().any(|d| url.contains(d))
    }

    pub async fn scrape(&self, url: &str) -> Result<ScrapedContent, Box<dyn std::error::Error + Send + Sync>> {
        let needs_rendering = Self::needs_rendering(url) || self.screenshot_mode;
        
        if needs_rendering {
            return self.scrape_with_browser(url).await;
        }

        let response = self.client
            .get(url)
            .send()
            .await?;

        let html = response.text().await?;

        let json_data = self.extract_json_from_html(&html);

        Ok(ScrapedContent {
            html: html.clone(),
            text: Some(self.extract_text(&html)),
            json_data,
            screenshot: None,
            needs_rendering: false,
            url: url.to_string(),
        })
    }

    async fn scrape_with_browser(&self, url: &str) -> Result<ScrapedContent, Box<dyn std::error::Error + Send + Sync>> {
        let screenshot = if self.screenshot_mode {
            Some(self.take_screenshot(url).await?)
        } else {
            None
        };

        let html = self.fetch_html_with_js(url).await?;

        let json_data = self.extract_json_from_html(&html);

        Ok(ScrapedContent {
            html: html.clone(),
            text: Some(self.extract_text(&html)),
            json_data,
            screenshot,
            needs_rendering: true,
            url: url.to_string(),
        })
    }

    async fn fetch_html_with_js(&self, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let output = std::process::Command::new("npx")
            .args(&["playwright", "screenshot", "--wait-for-timeout", "3000", url, "-"])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let html = String::from_utf8_lossy(&out.stdout).to_string();
                Ok(html)
            }
            _ => {
                let response = self.client
                    .get(url)
                    .send()
                    .await?;
                Ok(response.text().await?)
            }
        }
    }

    async fn take_screenshot(&self, url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let output = std::process::Command::new("npx")
            .args(&["playwright", "screenshot", "--wait-for-timeout", "3000", url, "-"])
            .output()?;

        Ok(output.stdout)
    }

    fn extract_json_from_html(&self, html: &str) -> Option<serde_json::Value> {
        if let Some(start) = html.find("__NEXT_DATA__\" type=\"application/json\">") {
            if let Some(end) = html[start..].find("</script>") {
                let json_str = &html[start + 36..start + end];
                return serde_json::from_str(json_str).ok();
            }
        }

        if let Some(start) = html.find("window.__INITIAL_STATE__ = ") {
            let end = html[start..].find(';').or(html[start..].find("</script>"))?;
            let json_str = &html[start + 27..start + end];
            return serde_json::from_str(json_str).ok();
        }

        None
    }

    fn extract_text(&self, html: &str) -> String {
        let mut text = String::new();
        let mut in_script = false;
        let mut in_style = false;
        let mut buffer = Vec::new();
        let chars: Vec<char> = html.chars().collect();
        
        for i in 0..chars.len() {
            if chars[i..].starts_with(&['<', 's', 'c', 'r', 'i', 'p', 't'][..]) {
                in_script = true;
            } else if chars[i..].starts_with(&['<', '/', 's', 'c', 'r', 'i', 'p', 't'][..]) {
                in_script = false;
            } else if chars[i..].starts_with(&['<', 's', 't', 'y', 'l', 'e'][..]) {
                in_style = true;
            } else if chars[i..].starts_with(&['<', '/', 's', 't', 'y', 'l', 'e'][..]) {
                in_style = false;
            } else if chars[i] == '<' {
                if !buffer.is_empty() && !in_script && !in_style {
                    text.push_str(&buffer.iter().collect::<String>().trim());
                    text.push(' ');
                    buffer.clear();
                }
            } else if !in_script && !in_style {
                buffer.push(chars[i]);
            }
        }
        
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

impl Default for WebScraper {
    fn default() -> Self {
        Self::new()
    }
}
