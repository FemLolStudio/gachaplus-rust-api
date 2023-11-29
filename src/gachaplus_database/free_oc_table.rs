use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    MySql, MySqlPool, Pool,
};
use std::sync::Arc;

pub struct FreeOcTable {
    pool: Arc<Pool<MySql>>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct FreeOc {
    pub accountx: String,
    owner: u64,
    pub secretid: String,
    pub mycode: String,
    #[serde(with = "ts_seconds")]
    pub createdate: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updatedate: DateTime<Utc>,
}

impl FreeOcTable {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
    pub async fn get_ocs(&self) -> Result<Vec<FreeOc>, sqlx::Error> {
        sqlx::query_as!(FreeOc, "SELECT * FROM `freeoc`")
            .fetch_all(&self.pool as &MySqlPool)
            .await
    }
}
