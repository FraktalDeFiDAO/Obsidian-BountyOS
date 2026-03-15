use async_trait::async_trait;
use obsidian_domain::{Bounty, BountyStatus, BountyType, Platform};
use sqlx::{sqlite::SqlitePoolOptions, postgres::PgPoolOptions, Row};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type DbResult<T> = Result<T, DbError>;

pub enum Database {
    Sqlite(SqliteDatabase),
    Postgres(PostgresDatabase),
}

impl Database {
    pub async fn new(url: &str) -> DbResult<Self> {
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            Ok(Database::Postgres(PostgresDatabase::new(url).await?))
        } else if url.starts_with("sqlite://") {
            Ok(Database::Sqlite(SqliteDatabase::new(url).await?))
        } else {
            let sqlite_url = format!("sqlite://{}", url);
            Ok(Database::Sqlite(SqliteDatabase::new(&sqlite_url).await?))
        }
    }

    pub async fn init(&self) -> DbResult<()> {
        match self {
            Database::Sqlite(db) => db.init().await,
            Database::Postgres(db) => db.init().await,
        }
    }
}

#[async_trait]
impl BountyRepository for Database {
    async fn upsert_bounty(&self, bounty: &Bounty) -> DbResult<()> {
        match self {
            Database::Sqlite(db) => db.upsert_bounty(bounty).await,
            Database::Postgres(db) => db.upsert_bounty(bounty).await,
        }
    }

    async fn get_bounty(&self, id: &str) -> DbResult<Option<Bounty>> {
        match self {
            Database::Sqlite(db) => db.get_bounty(id).await,
            Database::Postgres(db) => db.get_bounty(id).await,
        }
    }

    async fn list_bounties(&self, limit: usize, offset: usize) -> DbResult<Vec<Bounty>> {
        match self {
            Database::Sqlite(db) => db.list_bounties(limit, offset).await,
            Database::Postgres(db) => db.list_bounties(limit, offset).await,
        }
    }

    async fn list_bounties_filtered(
        &self,
        platform: Option<&Platform>,
        status: Option<&BountyStatus>,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>> {
        match self {
            Database::Sqlite(db) => db.list_bounties_filtered(platform, status, limit, offset).await,
            Database::Postgres(db) => db.list_bounties_filtered(platform, status, limit, offset).await,
        }
    }

    async fn count_bounties(&self) -> DbResult<i64> {
        match self {
            Database::Sqlite(db) => db.count_bounties().await,
            Database::Postgres(db) => db.count_bounties().await,
        }
    }
}

pub struct SqliteDatabase {
    pool: sqlx::SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(url: &str) -> DbResult<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn init(&self) -> DbResult<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS bounties (
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
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

pub struct PostgresDatabase {
    pool: sqlx::PgPool,
}

impl PostgresDatabase {
    pub async fn new(url: &str) -> DbResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn init(&self) -> DbResult<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS bounties (
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
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
pub trait BountyRepository: Send + Sync {
    async fn upsert_bounty(&self, bounty: &Bounty) -> DbResult<()>;
    async fn get_bounty(&self, id: &str) -> DbResult<Option<Bounty>>;
    async fn list_bounties(&self, limit: usize, offset: usize) -> DbResult<Vec<Bounty>>;
    async fn list_bounties_filtered(
        &self,
        platform: Option<&Platform>,
        status: Option<&BountyStatus>,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>>;
    async fn count_bounties(&self) -> DbResult<i64>;
}

#[async_trait]
impl BountyRepository for SqliteDatabase {
    async fn upsert_bounty(&self, bounty: &Bounty) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO bounties (id, external_id, platform, title, description, url, 
                bounty_type, status, reward_min, reward_max, reward_currency, 
                skills, tags, metadata, created_at, updated_at, synced_at, expires_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
                expires_at = excluded.expires_at
            "#,
        )
        .bind(bounty.id.to_string())
        .bind(&bounty.external_id)
        .bind(bounty.platform.as_str())
        .bind(&bounty.title)
        .bind(&bounty.description)
        .bind(&bounty.url)
        .bind(bounty.bounty_type.as_str())
        .bind(bounty.status.as_str())
        .bind(bounty.reward_min.map(|r| r.to_string()))
        .bind(bounty.reward_max.map(|r| r.to_string()))
        .bind(&bounty.reward_currency)
        .bind(serde_json::to_string(&bounty.skills).unwrap_or_default())
        .bind(serde_json::to_string(&bounty.tags).unwrap_or_default())
        .bind(serde_json::to_string(&bounty.metadata).unwrap_or_default())
        .bind(bounty.created_at.to_rfc3339())
        .bind(bounty.updated_at.to_rfc3339())
        .bind(bounty.synced_at.to_rfc3339())
        .bind(bounty.expires_at.map(|d| d.to_rfc3339()))
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_bounty(&self, id: &str) -> DbResult<Option<Bounty>> {
        let row = sqlx::query("SELECT * FROM bounties WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| Self::row_to_bounty(r)))
    }

    async fn list_bounties(&self, limit: usize, offset: usize) -> DbResult<Vec<Bounty>> {
        self.list_bounties_filtered(None, None, limit, offset).await
    }

    async fn list_bounties_filtered(
        &self,
        platform: Option<&Platform>,
        status: Option<&BountyStatus>,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>> {
        let mut query = String::from("SELECT * FROM bounties WHERE 1=1");
        let mut param_idx = 1;
        
        if platform.is_some() {
            query.push_str(&format!(" AND platform = ?{}", param_idx));
            param_idx += 1;
        }
        if status.is_some() {
            query.push_str(&format!(" AND status = ?{}", param_idx));
            param_idx += 1;
        }
        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}", param_idx, param_idx + 1));

        let mut q = sqlx::query(&query);
        if let Some(p) = platform {
            q = q.bind(p.as_str());
        }
        if let Some(s) = status {
            q = q.bind(s.as_str());
        }
        q = q.bind(limit as i64).bind(offset as i64);

        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| Self::row_to_bounty(r)).collect())
    }

    async fn count_bounties(&self) -> DbResult<i64> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bounties")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }
}

impl SqliteDatabase {
    fn row_to_bounty(row: sqlx::sqlite::SqliteRow) -> Bounty {
        let id: String = row.get("id");
        let platform_str: String = row.get("platform");
        let bounty_type_str: String = row.get("bounty_type");
        let status_str: String = row.get("status");
        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");
        let synced_at_str: String = row.get("synced_at");
        let skills_str: String = row.try_get("skills").unwrap_or_else(|_| "[]".to_string());
        let tags_str: String = row.try_get("tags").unwrap_or_else(|_| "[]".to_string());
        let metadata_str: String = row.try_get("metadata").unwrap_or_else(|_| "{}".to_string());
        let reward_min_str: String = row.try_get("reward_min").unwrap_or_else(|_| String::new());
        let reward_max_str: String = row.try_get("reward_max").unwrap_or_else(|_| String::new());
        let expires_at_str: String = row.try_get("expires_at").unwrap_or_else(|_| String::new());

        Bounty {
            id: uuid::Uuid::parse_str(&id).unwrap_or_default(),
            external_id: row.get("external_id"),
            platform: Platform::parse(&platform_str),
            title: row.get("title"),
            description: row.try_get("description").unwrap_or_default(),
            url: row.get("url"),
            bounty_type: BountyType::parse(&bounty_type_str),
            status: BountyStatus::parse(&status_str),
            reward_min: reward_min_str.parse().ok(),
            reward_max: reward_max_str.parse().ok(),
            reward_currency: row.try_get::<String, _>("reward_currency").ok().filter(|s| !s.is_empty()),
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
        }
    }
}

#[async_trait]
impl BountyRepository for PostgresDatabase {
    async fn upsert_bounty(&self, bounty: &Bounty) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO bounties (id, external_id, platform, title, description, url, 
                bounty_type, status, reward_min, reward_max, reward_currency, 
                skills, tags, metadata, created_at, updated_at, synced_at, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
             ON CONFLICT(platform, external_id) DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                url = EXCLUDED.url,
                status = EXCLUDED.status,
                reward_min = EXCLUDED.reward_min,
                reward_max = EXCLUDED.reward_max,
                reward_currency = EXCLUDED.reward_currency,
                metadata = EXCLUDED.metadata,
                updated_at = EXCLUDED.updated_at,
                synced_at = EXCLUDED.synced_at,
                expires_at = EXCLUDED.expires_at
            "#,
        )
        .bind(bounty.id.to_string())
        .bind(&bounty.external_id)
        .bind(bounty.platform.as_str())
        .bind(&bounty.title)
        .bind(&bounty.description)
        .bind(&bounty.url)
        .bind(bounty.bounty_type.as_str())
        .bind(bounty.status.as_str())
        .bind(bounty.reward_min.map(|r| r.to_string()))
        .bind(bounty.reward_max.map(|r| r.to_string()))
        .bind(&bounty.reward_currency)
        .bind(serde_json::to_string(&bounty.skills).unwrap_or_default())
        .bind(serde_json::to_string(&bounty.tags).unwrap_or_default())
        .bind(serde_json::to_string(&bounty.metadata).unwrap_or_default())
        .bind(bounty.created_at.to_rfc3339())
        .bind(bounty.updated_at.to_rfc3339())
        .bind(bounty.synced_at.to_rfc3339())
        .bind(bounty.expires_at.map(|d| d.to_rfc3339()))
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_bounty(&self, id: &str) -> DbResult<Option<Bounty>> {
        let row = sqlx::query("SELECT * FROM bounties WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| Self::row_to_bounty(r)))
    }

    async fn list_bounties(&self, limit: usize, offset: usize) -> DbResult<Vec<Bounty>> {
        self.list_bounties_filtered(None, None, limit, offset).await
    }

    async fn list_bounties_filtered(
        &self,
        platform: Option<&Platform>,
        status: Option<&BountyStatus>,
        limit: usize,
        offset: usize,
    ) -> DbResult<Vec<Bounty>> {
        let mut query = String::from("SELECT * FROM bounties WHERE 1=1");
        
        if platform.is_some() {
            query.push_str(" AND platform = $1");
        }
        if status.is_some() {
            query.push_str(if platform.is_some() { " AND status = $2" } else { " AND status = $1" });
        }
        query.push_str(" ORDER BY created_at DESC LIMIT $3 OFFSET $4");

        let rows = if platform.is_some() && status.is_some() {
            sqlx::query(&query)
                .bind(platform.unwrap().as_str())
                .bind(status.unwrap().as_str())
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
        } else if platform.is_some() {
            sqlx::query(&query)
                .bind(platform.unwrap().as_str())
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
        } else if status.is_some() {
            sqlx::query(&query)
                .bind(status.unwrap().as_str())
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(&query)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
        };

        Ok(rows.into_iter().map(|r| Self::row_to_bounty(r)).collect())
    }

    async fn count_bounties(&self) -> DbResult<i64> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bounties")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }
}

impl PostgresDatabase {
    fn row_to_bounty(row: sqlx::postgres::PgRow) -> Bounty {
        let id: String = row.get("id");
        let platform_str: String = row.get("platform");
        let bounty_type_str: String = row.get("bounty_type");
        let status_str: String = row.get("status");
        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");
        let synced_at_str: String = row.get("synced_at");
        let skills_str: String = row.try_get("skills").unwrap_or_else(|_| "[]".to_string());
        let tags_str: String = row.try_get("tags").unwrap_or_else(|_| "[]".to_string());
        let metadata_str: String = row.try_get("metadata").unwrap_or_else(|_| "{}".to_string());
        let reward_min_str: String = row.try_get("reward_min").unwrap_or_else(|_| String::new());
        let reward_max_str: String = row.try_get("reward_max").unwrap_or_else(|_| String::new());
        let expires_at_str: String = row.try_get("expires_at").unwrap_or_else(|_| String::new());

        Bounty {
            id: uuid::Uuid::parse_str(&id).unwrap_or_default(),
            external_id: row.get("external_id"),
            platform: Platform::parse(&platform_str),
            title: row.get("title"),
            description: row.try_get("description").unwrap_or_default(),
            url: row.get("url"),
            bounty_type: BountyType::parse(&bounty_type_str),
            status: BountyStatus::parse(&status_str),
            reward_min: reward_min_str.parse().ok(),
            reward_max: reward_max_str.parse().ok(),
            reward_currency: row.try_get::<String, _>("reward_currency").ok().filter(|s| !s.is_empty()),
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
        }
    }
}
