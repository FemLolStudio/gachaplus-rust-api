use axum::{
    body::Body,
    extract::OriginalUri,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::http_handler::response_manager;

pub async fn answer_200(
    OriginalUri(path): OriginalUri,
    request: Request<Body>,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    if response.status() == StatusCode::OK || response.status() == StatusCode::PERMANENT_REDIRECT {
        return response;
    } else {
        if !path.to_string().starts_with("/GPscripts/") {
            return response;
        }
        #[cfg(not(debug_assertions))]
        return response_manager::ResponseManager::new_error().into_response();
        #[cfg(debug_assertions)]
        {
            let (parts, body) = response.into_parts();
            let (_, bodytxt) = super::log::get_body(body).await;
            let msg = bodytxt.unwrap_or(String::new());
            let long_code = parts.status.to_string();
            let mut re = response_manager::ResponseManager::new_error()
                .add("code", parts.status.as_str())
                .add("long_code", &long_code);
            if !msg.is_empty() {
                re = re.add("msg", &msg);
            }
            re.into_response()
        }
    }
}
