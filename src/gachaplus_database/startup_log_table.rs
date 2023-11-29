use std::sync::Arc;

use sqlx::{MySql, MySqlPool, Pool};

pub struct StartupLogTable {
    pool: Arc<Pool<MySql>>,
}

impl StartupLogTable {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
    pub async fn insert(
        &self,
        platform: String,
        version: String,
        xbits: u8,
    ) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO `startup_log`(`platform`, `version`, `xbits`) VALUES (?,?,?)",
            platform,
            version,
            xbits
        )
        .execute(&self.pool as &MySqlPool)
        .await
    }
}
