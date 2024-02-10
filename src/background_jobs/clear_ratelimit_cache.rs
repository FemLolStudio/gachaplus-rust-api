use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use chrono::Utc;
use inline_colorization::*;
use tokio::time::sleep;

use crate::http_handler::AppState;

pub async fn cleanup_ratelimit_cache(app_state: Arc<AppState>) {
    loop {
        let now = Instant::now();
        let mut count_before = 0;
        let mut count_after = 0;
        for (_, (hash_map, _)) in app_state.rate_limit.iter() {
            let now = Instant::now();
            let mut cache = hash_map.lock().await;
            count_before += cache.len();
            cache.retain(|_, &mut timestamp| timestamp > now);
            count_after += cache.len();
        }
        let delay_in_ms = now.elapsed().as_micros() as f64 / 1000f64;

        if count_before - count_after > 0 {
            println!(
                "{}{}\tRateLimitCleaner: {} ip removed!\tDelay: {:.3} ms{}",
                color_bright_black,
                Utc::now().format("[%H:%M:%S]"),
                count_before - count_after,
                delay_in_ms,
                color_white,
            );
        }

        sleep(Duration::from_secs(60)).await;
    }
}
