use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::HeaderMap;
use tokio::sync::Mutex;

use crate::http_handler::{ip_manager, AppState};

pub async fn rate_limit_middleware(
    State(app_state): State<Arc<AppState>>,

    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,

    req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path();

    if let Some((cache, delay)) = app_state.rate_limit.get(&path) {
        let ip = ip_manager::get_user_ip(addr, headers);

        let mut cache = cache.lock().await;
        if let Some(timestamp) = cache.get(&ip) {
            if *timestamp > Instant::now() {
                // update
                cache.insert(ip, Instant::now() + *delay);
                //return
                return StatusCode::TOO_MANY_REQUESTS.into_response();
            }
        }

        // update
        cache.insert(ip, Instant::now() + *delay);
    }

    next.run(req).await
}

fn create_rules() -> HashMap<&'static str, Duration> {
    #[cfg(not(debug_assertions))]
    {
        let mut rules = HashMap::new();
        rules.insert("/GPscripts/club_export.php", Duration::from_secs(3));
        rules.insert("/GPscripts/club_import.php", Duration::from_secs(2));
        rules.insert("/GPscripts/club_register.php", Duration::from_secs(60));
        rules.insert("/GPscripts/club_login.php", Duration::from_secs(10));
        rules.insert("/GPscripts/startup.php", Duration::from_secs(15));
        rules.insert("/GPscripts/randomcode.php", Duration::from_millis(200));
        rules
    }
    #[cfg(debug_assertions)]
    HashMap::new()
}

pub fn create_ratelimit() -> HashMap<&'static str, (Mutex<HashMap<String, Instant>>, Duration)> {
    let rules = create_rules();
    let mut cache = HashMap::new();
    for (path, dur) in rules {
        cache.insert(path, (Mutex::new(HashMap::new()), dur));
    }

    cache
}
