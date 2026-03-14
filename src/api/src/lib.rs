use async_graphql::{Object, Schema, SimpleObject, ID};
use obsidian_domain::Platform;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn bounties(&self) -> Vec<BountyGql> {
        vec![]
    }

    async fn bounty(&self, _id: ID) -> Option<BountyGql> {
        None
    }

    async fn platforms(&self) -> Vec<PlatformInfo> {
        vec![
            PlatformInfo { name: "github".to_string(), enabled: true },
            PlatformInfo { name: "gitcoin".to_string(), enabled: true },
            PlatformInfo { name: "hackerone".to_string(), enabled: false },
            PlatformInfo { name: "bugcrowd".to_string(), enabled: false },
        ]
    }

    async fn health(&self) -> Health {
        Health { status: "healthy".to_string() }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn sync_platform(&self, platform: String) -> SyncResult {
        SyncResult { 
            platform, 
            success: true, 
            bounties_found: 0,
            error: None 
        }
    }
}

#[derive(SimpleObject)]
pub struct BountyGql {
    pub id: ID,
    pub external_id: String,
    pub platform: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub bounty_type: String,
    pub status: String,
    pub reward_min: Option<f64>,
    pub reward_max: Option<f64>,
    pub reward_currency: Option<String>,
    pub skills: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(SimpleObject)]
pub struct PlatformInfo {
    pub name: String,
    pub enabled: bool,
}

#[derive(SimpleObject)]
pub struct SyncResult {
    pub platform: String,
    pub success: bool,
    pub bounties_found: i32,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct Health {
    pub status: String,
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription)
        .finish()
}
