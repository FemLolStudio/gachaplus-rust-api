use std::{error::Error, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, prelude::FromRow, MySql, MySqlPool, Pool};

use crate::character_code::CharacterCode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferDatas {
    pub accountx: String,
    pub datastring1: String,
    pub datastring2: String,
    pub datastring3: String,
    pub datastring4: String,
    pub datastring5: String,
    pub datastring6: String,
    pub datastring7: String,
    pub datastring8: String,
    pub datastring9: String,
    pub datastring10: String,
    pub datastring11: String,
    pub datastring12: String,
    pub datastring13: String,
    pub datastring14: String,
    pub datastring15: String,
    pub datastring16: String,
    pub datastring17: String,
    pub datastring18: String,
    pub datastring19: String,
    pub datastring20: String,
    pub extranamestring: Option<String>,
    pub extraslotstring: Option<String>,
}
impl TransferDatas {
    pub fn is_invalid(&self) -> Option<String> {
        let accountx = self.accountx.parse::<u32>().unwrap_or_default();
        if accountx < 100_000_000 || accountx > 999_999_999 {
            Some(format!("Transferdata invalid: `accountx`"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring9) {
            Some(format!("Transferdata invalid: `datastring9` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring10) {
            Some(format!("Transferdata invalid: `datastring10` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring11) {
            Some(format!("Transferdata invalid: `datastring11` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring12) {
            Some(format!("Transferdata invalid: `datastring12` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring13) {
            Some(format!("Transferdata invalid: `datastring13` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring14) {
            Some(format!("Transferdata invalid: `datastring14` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring15) {
            Some(format!("Transferdata invalid: `datastring15` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring16) {
            Some(format!("Transferdata invalid: `datastring16` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring17) {
            Some(format!("Transferdata invalid: `datastring17` ({error})"))
        } else if let Err(error) = CharacterCode::new_from_code(&self.datastring18) {
            Some(format!("Transferdata invalid: `datastring18` ({error})"))
        } else {
            None
        }
    }
}

impl From<Vec<u8>> for TransferDatas {
    fn from(value: Vec<u8>) -> Self {
        serde_json::from_slice(&value).unwrap()
    }
}

#[derive(FromRow, Debug)]
pub struct TransferDatasRow {
    pub accountx: u32,
    pub data: TransferDatas,
    pub used: u16,
    pub regdate: DateTime<Utc>,
}

pub struct TransferDatasTable {
    pool: Arc<Pool<MySql>>,
}

impl TransferDatasTable {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
    pub async fn get(&self, accountx: u32) -> Result<TransferDatasRow, sqlx::Error> {
        sqlx::query_as!(
            TransferDatasRow,
            "SELECT * FROM `transfer` WHERE `accountx`= ?",
            accountx
        )
        .fetch_one(&self.pool as &MySqlPool)
        .await
    }
    pub async fn update(&self, accountx: u32) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE `transfer` SET `used`=`used`+1 WHERE `accountx`= ?",
            accountx
        )
        .execute(&self.pool as &MySqlPool)
        .await
    }
    pub async fn insert_or_update(
        &self,
        datas: TransferDatas,
    ) -> Result<MySqlQueryResult, Box<dyn Error>> {
        let data_json = serde_json::to_string(&datas)?;
        let result = sqlx::query!(
            "INSERT INTO `transfer`(`accountx`, `data`) VALUES (?,?) ON DUPLICATE KEY UPDATE `data`=?", 
            datas.accountx,
            &data_json,
            &data_json
        )
            .execute(&self.pool as &MySqlPool)
            .await?;
        Ok(result)
    }
}
