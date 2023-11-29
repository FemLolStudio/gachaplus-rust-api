use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, MySqlPool, Pool};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct LatestVersionRow {
    pub id: u32,
    pub version: String,
    pub checksum: String,
    pub url: String,
    pub regdate: DateTime<Utc>,
}

pub struct LatestVersionTable {
    pool: Arc<Pool<MySql>>,
}
impl LatestVersionTable {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
    pub async fn get_latest_version(&self) -> Result<LatestVersionRow, sqlx::Error> {
        sqlx::query_as!(
            LatestVersionRow,
            "SELECT * FROM `latestversion` ORDER BY `id` DESC LIMIT 1"
        )
        .fetch_one(&self.pool as &MySqlPool)
        .await
    }
}
