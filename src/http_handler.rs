use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::response::Redirect;
use axum::{middleware, routing, Router};
use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, RwLock};
use tower_http::services::ServeDir;

use crate::gachaplus_database::short_log_table::ShortLog;

use self::middlewares::ratelimit::create_ratelimit;

use super::gachaplus_database::free_oc_table::FreeOc;
use super::gachaplus_database::GachaPlusDatabase;

mod ip_manager;
mod middlewares;
mod response_manager;
use middlewares::*;
mod handlers;
use handlers::*;

pub struct AppState {
    pub database: GachaPlusDatabase,
    pub oc_chache: RwLock<Vec<FreeOc>>,
    pub log_queue: Mutex<Vec<ShortLog>>,
    pub rate_limit: HashMap<&'static str, (Mutex<HashMap<String, Instant>>, Duration)>,
    pub startup_time: DateTime<Utc>,
}
impl AppState {
    pub async fn new(database_url: String) -> Arc<Self> {
        let database = GachaPlusDatabase::new(database_url).await;
        let oc_chache = RwLock::new(Vec::new());
        let log_queue = Mutex::new(Vec::new());
        let rate_limit = create_ratelimit();
        let startup_time = Utc::now();
        let app_state = AppState {
            database,
            oc_chache,
            log_queue,
            rate_limit,
            startup_time,
        };
        Arc::new(app_state)
    }
}

pub async fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest_service("/files", ServeDir::new("files"))
        .route(
            "/",
            routing::any(Redirect::permanent(
                "https://femlol-studio.itch.io/gacha-plus",
            )),
        )
        .route("/info", routing::get(stat::get_info))
        .route("/test", routing::get(hello_world::get_hello_world))
        .route(
            "/GPscripts/latestversion_and_checksum.php",
            routing::any(version::verion_and_checksum),
        )
        .route(
            "/GPscripts/startup.php",
            routing::post(startup::startup_request),
        )
        .route(
            "/GPscripts/randomcode.php",
            routing::post(random_character::get_random_oc),
        )
        .route(
            "/GPscripts/club_import.php",
            routing::post(character::get_oc),
        )
        .route(
            "/GPscripts/club_export.php",
            routing::post(character::add_oc),
        )
        .route(
            "/GPscripts/club_login.php",
            routing::post(transfer_datas::get_transfer_datas),
        )
        .route(
            "/GPscripts/club_register.php",
            routing::post(transfer_datas::add_transfer_datas),
        )
        .with_state(app_state.clone())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            logdb::log_db,
        ))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            ratelimit::rate_limit_middleware,
        ))
        .layer(middleware::from_fn(fake_request::fake_request_middleware))
        .layer(middleware::from_fn(log::log))
        .layer(middleware::from_fn(answer_200::answer_200))
}
