use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use obsidian_db::{Database, BountyRepository};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn bounties(
        &self,
        ctx: &Context<'_>,
        platform: Option<String>,
        status: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> async_graphql::Result<Vec<BountyGql>> {
        let db = ctx.data::<Database>()?;
        let bounties = db
            .list_bounties(limit.unwrap_or(50) as usize, offset.unwrap_or(0) as usize)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(bounties
            .into_iter()
            .filter(|b| {
                let platform_match = platform
                    .as_ref()
                    .map(|p| b.platform.as_str() == p)
                    .unwrap_or(true);
                let status_match = status
                    .as_ref()
                    .map(|s| b.status.as_str() == s)
                    .unwrap_or(true);
                platform_match && status_match
            })
            .map(|b| BountyGql {
                id: ID(b.id.to_string()),
                external_id: b.external_id,
                platform: b.platform.as_str().to_string(),
                title: b.title,
                description: b.description,
                url: b.url,
                bounty_type: b.bounty_type.as_str().to_string(),
                status: b.status.as_str().to_string(),
                reward_min: b.reward_min.map(|r| r.to_string().parse().unwrap_or(0.0)),
                reward_max: b.reward_max.map(|r| r.to_string().parse().unwrap_or(0.0)),
                reward_currency: b.reward_currency,
                skills: b.skills,
                tags: b.tags,
            })
            .collect())
    }

    async fn bounty(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Option<BountyGql>> {
        let db = ctx.data::<Database>()?;
        let bounty = db
            .get_bounty(&id.to_string())
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(bounty.map(|b| BountyGql {
            id: ID(b.id.to_string()),
            external_id: b.external_id,
            platform: b.platform.as_str().to_string(),
            title: b.title,
            description: b.description,
            url: b.url,
            bounty_type: b.bounty_type.as_str().to_string(),
            status: b.status.as_str().to_string(),
            reward_min: b.reward_min.map(|r| r.to_string().parse().unwrap_or(0.0)),
            reward_max: b.reward_max.map(|r| r.to_string().parse().unwrap_or(0.0)),
            reward_currency: b.reward_currency,
            skills: b.skills,
            tags: b.tags,
        }))
    }

    async fn platforms(&self) -> Vec<PlatformInfo> {
        vec![
            PlatformInfo { name: "github".to_string(), enabled: true },
            PlatformInfo { name: "gitcoin".to_string(), enabled: true },
            PlatformInfo { name: "hackerone".to_string(), enabled: false },
            PlatformInfo { name: "bugcrowd".to_string(), enabled: false },
            PlatformInfo { name: "laborx".to_string(), enabled: false },
            PlatformInfo { name: "dework".to_string(), enabled: false },
        ]
    }

    async fn stats(&self, ctx: &Context<'_>) -> async_graphql::Result<Stats> {
        let db = ctx.data::<Database>()?;
        let total = db
            .count_bounties()
            .await
            .map_err(|e: obsidian_db::DbError| async_graphql::Error::new(e.to_string()))?;

        Ok(Stats { total })
    }

    async fn health(&self) -> Health {
        Health { status: "healthy".to_string(), version: "0.1.0".to_string() }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn sync_platform(
        &self,
        _platform: String,
    ) -> async_graphql::Result<SyncResult> {
        Ok(SyncResult {
            platform: _platform,
            success: true,
            bounties_found: 0,
            error: None,
        })
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
pub struct Stats {
    pub total: i64,
}

#[derive(SimpleObject)]
pub struct Health {
    pub status: String,
    pub version: String,
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(db: Database) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}
