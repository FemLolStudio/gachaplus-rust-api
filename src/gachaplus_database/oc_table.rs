use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    MySql, MySqlPool, Pool,
};
use std::{error::Error, sync::Arc};

pub struct OcTable {
    pool: Arc<Pool<MySql>>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Oc {
    pub accountx: String,
    pub secretid: String,
    pub mycode: String,
    pub used: u16,
    pub createdate: DateTime<Utc>,
    pub updatedate: DateTime<Utc>,
}
impl Oc {
    pub fn new(accountx: String, secretid: String, mycode: String) -> Self {
        Self {
            accountx,
            secretid,
            mycode,
            createdate: Utc::now(),
            updatedate: Utc::now(),
            used: 0,
        }
    }
}

impl OcTable {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
    pub async fn get_oc(&self, accountx: &str) -> Result<Oc, sqlx::Error> {
        sqlx::query_as!(Oc, "SELECT * FROM `oc` WHERE `accountx` = ?", accountx)
            .fetch_one(&self.pool as &MySqlPool)
            .await
    }
    pub async fn insert_or_update_oc(
        &self,
        oc: Oc,
    ) -> Result<sqlx::mysql::MySqlQueryResult, Box<dyn Error>> {
        let row_option = sqlx::query!(
            "SELECT `secretid` FROM `oc` WHERE `accountx`=?",
            oc.accountx
        )
        .fetch_optional(&self.pool as &MySqlPool)
        .await?;
        let res = match row_option {
            Some(row) => {
                if row.secretid != oc.secretid {
                    return Err(String::from("No access").into());
                }
                sqlx::query!(
                    "UPDATE `oc` SET `mycode`= ? WHERE `accountx`= ? AND `secretid` = ?",
                    oc.mycode,
                    oc.accountx,
                    oc.secretid
                )
                .execute(&self.pool as &MySqlPool)
                .await?
            }
            None => {
                sqlx::query!(
                    "INSERT INTO `oc`(`accountx`, `secretid`, `mycode`) VALUES (?, ?, ?)",
                    oc.accountx,
                    oc.secretid,
                    oc.mycode
                )
                .execute(&self.pool as &MySqlPool)
                .await?
            }
        };
        Ok(res)
    }
}
