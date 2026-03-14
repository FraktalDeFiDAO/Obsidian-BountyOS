use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use rusqlite::{params, Connection};
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Query(#[from] rusqlite::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type DbResult<T> = Result<T, DbError>;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> DbResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS bounties (
                id TEXT PRIMARY KEY,
                external_id TEXT NOT NULL,
                platform TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT DEFAULT '',
                url TEXT NOT NULL,
                bounty_type TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active',
                reward_min TEXT,
                reward_max TEXT,
                reward_currency TEXT,
                skills TEXT DEFAULT '[]',
                tags TEXT DEFAULT '[]',
                metadata TEXT DEFAULT '{}',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                synced_at TEXT NOT NULL,
                expires_at TEXT,
                UNIQUE(platform, external_id)
            )",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

#[async_trait]
pub trait BountyRepository: Send + Sync {
    async fn upsert_bounty(&self, bounty: &Bounty) -> DbResult<()>;
    async fn get_bounty(&self, id: &str) -> DbResult<Option<Bounty>>;
    async fn list_bounties(&self, limit: usize, offset: usize) -> DbResult<Vec<Bounty>>;
    async fn list_bounties_by_platform(
        &self,
        platform: &Platform,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>>;
    async fn list_bounties_by_status(
        &self,
        status: &BountyStatus,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>>;
    async fn list_bounties_filtered(
        &self,
        platform: Option<&Platform>,
        status: Option<&BountyStatus>,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>>;
    async fn count_bounties(&self) -> DbResult<i64>;
    async fn count_bounties_by_platform(&self, platform: &Platform) -> DbResult<i64>;
    async fn count_bounties_by_status(&self, status: &BountyStatus) -> DbResult<i64>;
}

#[async_trait]
impl BountyRepository for Database {
    async fn upsert_bounty(&self, bounty: &Bounty) -> DbResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO bounties (id, external_id, platform, title, description, url, 
                bounty_type, status, reward_min, reward_max, reward_currency, 
                skills, tags, metadata, created_at, updated_at, synced_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
             ON CONFLICT(platform, external_id) DO UPDATE SET
                title = excluded.title,
                description = excluded.description,
                url = excluded.url,
                status = excluded.status,
                reward_min = excluded.reward_min,
                reward_max = excluded.reward_max,
                reward_currency = excluded.reward_currency,
                metadata = excluded.metadata,
                updated_at = excluded.updated_at,
                synced_at = excluded.synced_at,
                expires_at = excluded.expires_at",
            params![
                bounty.id.to_string(),
                bounty.external_id,
                bounty.platform.as_str(),
                bounty.title,
                bounty.description,
                bounty.url,
                bounty.bounty_type.as_str(),
                bounty.status.as_str(),
                bounty.reward_min.map(|r| r.to_string()),
                bounty.reward_max.map(|r| r.to_string()),
                bounty.reward_currency,
                serde_json::to_string(&bounty.skills).unwrap_or_default(),
                serde_json::to_string(&bounty.tags).unwrap_or_default(),
                serde_json::to_string(&bounty.metadata).unwrap_or_default(),
                bounty.created_at.to_rfc3339(),
                bounty.updated_at.to_rfc3339(),
                bounty.synced_at.to_rfc3339(),
                bounty.expires_at.map(|d| d.to_rfc3339()),
            ],
        )?;
        Ok(())
    }

    async fn get_bounty(&self, id: &str) -> DbResult<Option<Bounty>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM bounties WHERE id = ?")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_bounty(row)?))
        } else {
            Ok(None)
        }
    }

    async fn list_bounties(&self, limit: usize, offset: usize) -> DbResult<Vec<Bounty>> {
        self.list_bounties_filtered(None, None, limit, offset).await
    }

    async fn list_bounties_by_platform(
        &self,
        platform: &Platform,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>> {
        self.list_bounties_filtered(Some(platform), None, limit, offset)
            .await
    }

    async fn list_bounties_by_status(
        &self,
        status: &BountyStatus,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>> {
        self.list_bounties_filtered(None, Some(status), limit, offset)
            .await
    }

    async fn list_bounties_filtered(
        &self,
        platform: Option<&Platform>,
        status: Option<&BountyStatus>,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>> {
        let conn = self.conn.lock().unwrap();

        let mut sql = String::from("SELECT * FROM bounties WHERE 1=1");
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(p) = platform {
            sql.push_str(" AND platform = ?");
            params_vec.push(Box::new(p.as_str().to_string()));
        }
        if let Some(s) = status {
            sql.push_str(" AND status = ?");
            params_vec.push(Box::new(s.as_str().to_string()));
        }

        sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
        params_vec.push(Box::new(limit as i64));
        params_vec.push(Box::new(offset as i64));

        let mut stmt = conn.prepare(&sql)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();
        let mut rows = stmt.query(params_refs.as_slice())?;

        let mut bounties = Vec::new();
        while let Some(row) = rows.next()? {
            bounties.push(Self::row_to_bounty(row)?);
        }
        Ok(bounties)
    }

    async fn count_bounties(&self) -> DbResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM bounties", [], |row| row.get(0))?;
        Ok(count)
    }

    async fn count_bounties_by_platform(&self, platform: &Platform) -> DbResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM bounties WHERE platform = ?",
            [platform.as_str()],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    async fn count_bounties_by_status(&self, status: &BountyStatus) -> DbResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM bounties WHERE status = ?",
            [status.as_str()],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}

impl Database {
    fn row_to_bounty(row: &rusqlite::Row) -> DbResult<Bounty> {
        let id: String = row.get("id")?;
        let platform_str: String = row.get("platform")?;
        let bounty_type_str: String = row.get("bounty_type")?;
        let status_str: String = row.get("status")?;
        let created_at_str: String = row.get("created_at")?;
        let updated_at_str: String = row.get("updated_at")?;
        let synced_at_str: String = row.get("synced_at")?;
        let skills_str: String = row.get("skills").unwrap_or_else(|_| "[]".to_string());
        let tags_str: String = row.get("tags").unwrap_or_else(|_| "[]".to_string());
        let metadata_str: String = row.get("metadata").unwrap_or_else(|_| "{}".to_string());
        let reward_min_str: String = row.get("reward_min").unwrap_or_else(|_| String::new());
        let reward_max_str: String = row.get("reward_max").unwrap_or_else(|_| String::new());
        let expires_at_str: String = row.get("expires_at").unwrap_or_else(|_| String::new());

        Ok(Bounty {
            id: uuid::Uuid::parse_str(&id).unwrap_or_default(),
            external_id: row.get("external_id")?,
            platform: Platform::parse(&platform_str),
            title: row.get("title")?,
            description: row.get("description").unwrap_or_default(),
            url: row.get("url")?,
            bounty_type: BountyType::parse(&bounty_type_str),
            status: BountyStatus::parse(&status_str),
            reward_min: reward_min_str.parse().ok(),
            reward_max: reward_max_str.parse().ok(),
            reward_currency: row
                .get("reward_currency")
                .ok()
                .filter(|s: &String| !s.is_empty()),
            skills: serde_json::from_str(&skills_str).unwrap_or_default(),
            tags: serde_json::from_str(&tags_str).unwrap_or_default(),
            metadata: serde_json::from_str(&metadata_str).unwrap_or(serde_json::json!({})),
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            synced_at: chrono::DateTime::parse_from_rfc3339(&synced_at_str)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            expires_at: if expires_at_str.is_empty() {
                None
            } else {
                chrono::DateTime::parse_from_rfc3339(&expires_at_str)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .ok()
            },
        })
    }
}
