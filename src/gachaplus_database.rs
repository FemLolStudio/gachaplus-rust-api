use std::sync::Arc;

use chrono::Utc;
use inline_colorization::*;
use sqlx::mysql::MySqlPoolOptions;

pub mod free_oc_table;
pub mod latestversion_table;
pub mod oc_table;
pub mod short_log_table;
pub mod startup_log_table;
pub mod tranfer_datas_table;

pub struct GachaPlusDatabase {
    pub oc_table: oc_table::OcTable,
    pub oc_random_table: free_oc_table::FreeOcTable,
    pub short_log_table: short_log_table::ShortLogTable,
    pub startup_log_table: startup_log_table::StartupLogTable,
    pub tranfer_datas_table: tranfer_datas_table::TransferDatasTable,
    pub latestversion_table: latestversion_table::LatestVersionTable,
}

impl GachaPlusDatabase {
    pub async fn new(database_url: String) -> Self {
        let pool = match MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                println!("{color_cyan}{}{color_green}\tDatabase: ✅ Connection to the database is successful! ✅{color_white}",
                Utc::now().format("[%H:%M:%S]"),
            );
                pool
            }
            Err(err) => {
                println!(
                    "{color_red}{}\tDatabase: 🔥 Failed to connect to the database: {:?} 🔥",
                    Utc::now().format("[%H:%M:%S]"),
                    err
                );
                std::process::exit(1);
            }
        };

        let shared_pool = Arc::new(pool);

        GachaPlusDatabase {
            oc_table: oc_table::OcTable::new(shared_pool.clone()),
            oc_random_table: free_oc_table::FreeOcTable::new(shared_pool.clone()),
            short_log_table: short_log_table::ShortLogTable::new(shared_pool.clone()),
            startup_log_table: startup_log_table::StartupLogTable::new(shared_pool.clone()),
            tranfer_datas_table: tranfer_datas_table::TransferDatasTable::new(shared_pool.clone()),
            latestversion_table: latestversion_table::LatestVersionTable::new(shared_pool.clone()),
        }
    }
}
