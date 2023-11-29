use serde::{Deserialize, Serialize};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use sqlx::{MySql, MySqlPool, Pool};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct ShortLog {
    pub id: u32,
    pub address: u32,
    pub action: u8,
    pub regdate: DateTime<Utc>,
}
pub struct ShortLogTable {
    pool: Arc<Pool<MySql>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionEnum {
    OCexport = 1,
    OCimport = 2,
    OCrandom = 3,
    WINAPPgetversion = 4,
    ALLexport = 5,
    ALLimport = 6,
    Startups = 7,
}
impl ActionEnum {
    pub fn value(&self) -> u8 {
        match self {
            ActionEnum::OCexport => 1,
            ActionEnum::OCimport => 2,
            ActionEnum::OCrandom => 3,
            ActionEnum::WINAPPgetversion => 4,
            ActionEnum::ALLexport => 5,
            ActionEnum::ALLimport => 6,
            ActionEnum::Startups => 7,
        }
    }

    /*
    pub fn from_value(num: u32) -> Option<ActionEnum> {
        match num {
            1 => Some(ActionEnum::OCexport),
            2 => Some(ActionEnum::OCimport),
            3 => Some(ActionEnum::OCrandom),
            4 => Some(ActionEnum::WINAPPgetversion),
            5 => Some(ActionEnum::ALLexport),
            6 => Some(ActionEnum::ALLimport),
            7 => Some(ActionEnum::Startups),
            _ => None,
        }
    }
    */
}

impl ShortLog {
    pub fn new(address: u32, action: u8) -> Self {
        Self {
            id: 0,
            address,
            action,
            regdate: Utc::now(),
        }
    }
}

impl ShortLogTable {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
    pub async fn insert_log(
        &self,
        log: &ShortLog,
    ) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO `shortlog`(`address`, `action`, `regdate`) VALUES (?,?,?)",
            log.address,
            log.action,
            log.regdate
        )
        .execute(&self.pool as &MySqlPool)
        .await
    }
}
