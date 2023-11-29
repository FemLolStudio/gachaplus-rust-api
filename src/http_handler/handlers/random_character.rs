use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rand::seq::SliceRandom;

use super::super::{response_manager::ResponseManager, AppState};

#[axum::debug_handler]
pub async fn get_random_oc(State(app_state): State<Arc<AppState>>) -> Response {
    let reader = app_state.oc_chache.read().await;

    let mut rng = rand::thread_rng();
    let random_oc = reader.choose(&mut rng);

    match random_oc {
        Some(oc) => ResponseManager::new_ok()
            .add("accountx", &oc.accountx)
            .add("xmycode", &oc.mycode)
            .into_response(),

        None => (StatusCode::INTERNAL_SERVER_ERROR, "No character result").into_response(),
    }
}
