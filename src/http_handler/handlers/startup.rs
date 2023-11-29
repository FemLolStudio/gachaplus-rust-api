use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use serde::Deserialize;

use crate::http_handler::{response_manager::ResponseManager, AppState};

#[derive(Debug, Deserialize)]
pub struct StartupInput {
    version: String,
    platform: String,
    xbits: u8,
}

#[axum::debug_handler]
pub async fn startup_request(
    State(app_state): State<Arc<AppState>>,
    Form(input): Form<StartupInput>,
) -> Response {
    if input.version.len() > 7 || input.platform.len() > 7 {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid `version` or `platform` length!",
        )
            .into_response();
    }
    let req = app_state
        .database
        .startup_log_table
        .insert(input.platform, input.version, input.xbits)
        .await;
    if let Err(error) = req {
        return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
    }

    ResponseManager::new_ok().into_response()
}
