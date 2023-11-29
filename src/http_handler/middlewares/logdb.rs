use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{ConnectInfo, OriginalUri, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use hyper::HeaderMap;

use crate::{
    gachaplus_database::short_log_table::{ActionEnum, ShortLog},
    http_handler::{ip_manager, AppState},
};

pub async fn log_db(
    State(app_state): State<Arc<AppState>>,

    OriginalUri(path): OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,

    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

    if response.status() == StatusCode::OK {
        let code = match path.to_string().as_str() {
            "/GPscripts/latestversion_and_checksum.php" => Some(ActionEnum::WINAPPgetversion),
            "/GPscripts/startup.php" => Some(ActionEnum::Startups),
            "/GPscripts/randomcode.php" => Some(ActionEnum::OCrandom),
            "/GPscripts/club_import.php" => Some(ActionEnum::OCimport),
            "/GPscripts/club_export.php" => Some(ActionEnum::OCexport),
            "/GPscripts/club_login.php" => Some(ActionEnum::ALLimport),
            "/GPscripts/club_register.php" => Some(ActionEnum::ALLexport),
            _ => None,
        };
        if let Some(code) = code {
            let ip = ip_manager::get_user_ip(addr, headers);
            let address_option = ip_manager::ip_to_long(&ip);
            if let Some(address) = address_option {
                let log = ShortLog::new(address, code.value());
                //println!("{:?}", log);
                app_state.log_queue.lock().await.push(log);
            }
        }
    }

    response
}
