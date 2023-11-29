use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use chrono::Utc;
use inline_colorization::*;
use tokio::time::sleep;

use crate::http_handler::AppState;

pub async fn random_character_cache_service(app_state: Arc<AppState>) {
    loop {
        let ocs_res = app_state.database.oc_random_table.get_ocs().await;
        if let Ok(ocs) = ocs_res {
            let now = Instant::now();
            let ocs_len = ocs.len();
            {
                let mut writer = app_state.oc_chache.write().await;
                *writer = ocs;
            }
            let delay_in_ms = now.elapsed().as_micros() as f64 / 1000f64;
            println!(
                "{}{}\tRandom character: {} character in the random character cache 🙆\tDelay: {:.3} ms{}",
                color_bright_black,
                Utc::now().format("[%H:%M:%S]"),
                ocs_len,
                delay_in_ms,
                color_white
            );
        }
        sleep(Duration::from_secs(60)).await;
    }
}
