use std::sync::Arc;

use super::http_handler::AppState;
mod clear_ratelimit_cache;
mod random_character_cache;
mod write_out_log;

pub fn start(app_state: Arc<AppState>) {
    tokio::spawn(random_character_cache::random_character_cache_service(
        app_state.clone(),
    ));
    tokio::spawn(write_out_log::write_out_log_service(app_state.clone()));
    tokio::spawn(clear_ratelimit_cache::cleanup_ratelimit_cache(
        app_state.clone(),
    ));
}
