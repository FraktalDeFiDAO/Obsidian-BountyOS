use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    GitHub,
    Gitcoin,
    HackerOne,
    Bugcrowd,
    LaborX,
    DeWork,
    Custom(String),
}

impl Platform {
    pub fn as_str(&self) -> &str {
        match self {
            Platform::GitHub => "github",
            Platform::Gitcoin => "gitcoin",
            Platform::HackerOne => "hackerone",
            Platform::Bugcrowd => "bugcrowd",
            Platform::LaborX => "laborx",
            Platform::DeWork => "dework",
            Platform::Custom(s) => s,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "github" => Platform::GitHub,
            "gitcoin" => Platform::Gitcoin,
            "hackerone" => Platform::HackerOne,
            "bugcrowd" => Platform::Bugcrowd,
            "laborx" => Platform::LaborX,
            "dework" => Platform::DeWork,
            other => Platform::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BountyType {
    BugBounty,
    Task,
    Grant,
    Microtask,
    Hackathon,
    Bounty,
}

impl BountyType {
    pub fn as_str(&self) -> &str {
        match self {
            BountyType::BugBounty => "bugbounty",
            BountyType::Task => "task",
            BountyType::Grant => "grant",
            BountyType::Microtask => "microtask",
            BountyType::Hackathon => "hackathon",
            BountyType::Bounty => "bounty",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bugbounty" => BountyType::BugBounty,
            "task" => BountyType::Task,
            "grant" => BountyType::Grant,
            "microtask" => BountyType::Microtask,
            "hackathon" => BountyType::Hackathon,
            "bounty" => BountyType::Bounty,
            _ => BountyType::Bounty,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BountyStatus {
    Active,
    Closed,
    Expired,
    Draft,
    Paused,
}

impl BountyStatus {
    pub fn as_str(&self) -> &str {
        match self {
            BountyStatus::Active => "active",
            BountyStatus::Closed => "closed",
            BountyStatus::Expired => "expired",
            BountyStatus::Draft => "draft",
            BountyStatus::Paused => "paused",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => BountyStatus::Active,
            "closed" => BountyStatus::Closed,
            "expired" => BountyStatus::Expired,
            "draft" => BountyStatus::Draft,
            "paused" => BountyStatus::Paused,
            _ => BountyStatus::Active,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounty {
    pub id: Uuid,
    pub external_id: String,
    pub platform: Platform,
    pub title: String,
    pub description: String,
    pub url: String,
    pub bounty_type: BountyType,
    pub status: BountyStatus,
    pub reward_min: Option<Decimal>,
    pub reward_max: Option<Decimal>,
    pub reward_currency: Option<String>,
    pub skills: Vec<String>,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub synced_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl Bounty {
    pub fn new(
        external_id: String,
        platform: Platform,
        title: String,
        url: String,
        bounty_type: BountyType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            external_id,
            platform,
            title,
            description: String::new(),
            url,
            bounty_type,
            status: BountyStatus::Active,
            reward_min: None,
            reward_max: None,
            reward_currency: None,
            skills: Vec::new(),
            tags: Vec::new(),
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
            synced_at: now,
            expires_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub platform: Platform,
    pub enabled: bool,
    pub api_key: Option<String>,
    pub webhook_url: Option<String>,
    pub sync_interval_minutes: u32,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub last_sync_status: SyncStatus,
}

impl PlatformConfig {
    pub fn new(platform: Platform) -> Self {
        Self {
            platform,
            enabled: true,
            api_key: None,
            webhook_url: None,
            sync_interval_minutes: 15,
            last_sync_at: None,
            last_sync_status: SyncStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHistory {
    pub id: Uuid,
    pub platform: Platform,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: SyncStatus,
    pub bounties_found: i32,
    pub bounties_created: i32,
    pub bounties_updated: i32,
    pub bounties_closed: i32,
    pub error_message: Option<String>,
}

impl SyncHistory {
    pub fn new(platform: Platform) -> Self {
        Self {
            id: Uuid::new_v4(),
            platform,
            started_at: Utc::now(),
            completed_at: None,
            status: SyncStatus::Running,
            bounties_found: 0,
            bounties_created: 0,
            bounties_updated: 0,
            bounties_closed: 0,
            error_message: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.status = SyncStatus::Completed;
    }

    pub fn fail(&mut self, error: String) {
        self.completed_at = Some(Utc::now());
        self.status = SyncStatus::Failed;
        self.error_message = Some(error);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationChannel {
    System,
    Telegram,
    Discord,
    Email,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub channel: NotificationChannel,
    pub status: NotificationStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub id: Uuid,
    pub name: String,
    pub value: Decimal,
    pub platform: Option<Platform>,
    pub recorded_at: DateTime<Utc>,
}

impl Metric {
    pub fn new(name: String, value: Decimal, platform: Option<Platform>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            value,
            platform,
            recorded_at: Utc::now(),
        }
    }
}
