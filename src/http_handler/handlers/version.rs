use std::sync::Arc;

use axum::extract::State;

use crate::http_handler::AppState;

#[axum::debug_handler]
pub async fn verion_and_checksum(State(app_state): State<Arc<AppState>>) -> String {
    let res = app_state
        .database
        .latestversion_table
        .get_latest_version()
        .await;
    match res {
        Ok(latest) => format!("{}|{}|{}", latest.version, latest.checksum, latest.url),
        Err(_) => format!("0.0.0|error|error"),
    }
}
