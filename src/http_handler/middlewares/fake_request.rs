#[cfg(not(debug_assertions))]
use axum::{
    extract::OriginalUri,
    http::{HeaderMap, StatusCode, Uri},
    response::IntoResponse,
};
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn fake_request_middleware(
    #[cfg(not(debug_assertions))] headers: HeaderMap,
    #[cfg(not(debug_assertions))] OriginalUri(path): OriginalUri,

    request: Request,
    next: Next,
) -> Response {
    #[cfg(not(debug_assertions))]
    if let Some(error) = is_fake(&headers, &path) {
        //println!("{:?}", headers);
        (StatusCode::BAD_REQUEST, format!("FAKE REQUEST: {error}")).into_response()
    } else {
        next.run(request).await
    }
    #[cfg(debug_assertions)]
    next.run(request).await
}

#[cfg(not(debug_assertions))]
fn is_fake(headers: &HeaderMap, path: &Uri) -> Option<&'static str> {
    {
        if path.to_string() == "/GPscripts/latestversion_and_checksum.php" {
            return None;
        }
        if !path.to_string().starts_with("/GPscripts") {
            return None;
        }

        //user agent
        let user_agent_option = headers.get("User-Agent");
        if let Some(user_agent) = user_agent_option.and_then(|ua| ua.to_str().ok()) {
            if !user_agent.contains("Mozilla/5.0") {
                return Some("'User-Agent' - 'Mozilla/5.0' missing");
            }
            if !user_agent.contains("AdobeAIR/33.0") && !user_agent.contains("AdobeAIR/32.0") {
                return Some("'User-Agent' - 'AdobeAIR/33.0'/'AdobeAIR/32.0'");
            }
            if !user_agent.contains("AppleWebKit/533.19.4 (KHTML, like Gecko)") {
                return Some("'User-Agent' - 'AppleWebKit/533.19.4 (KHTML, like Gecko)'");
            }
            if !user_agent.contains("Android") && !user_agent.contains("Windows") {
                return Some("'User-Agent' - 'Android'/'Windows'");
            }
        } else {
            return Some("No 'User-Agent'");
        }

        //refer
        let refer_option = headers.get("Refer");
        let referer_option = headers.get("Referer");
        if let Some(refer) = refer_option.and_then(|ua| ua.to_str().ok()) {
            if refer != "app:/gacha_plus.swf" && refer != "app:/gacha_club.swf" {
                return Some("'Refer' - 'app:/gacha_club.swf'/'app:/gacha_plus.swf'");
            }
        } else if let Some(referer) = referer_option.and_then(|ua| ua.to_str().ok()) {
            if referer != "app:/gacha_plus.swf" && referer != "app:/gacha_club.swf" {
                return Some("'Referer' - 'app:/gacha_club.swf'/'app:/gacha_plus.swf'");
            }
        } else {
            println!("{:?}", headers);
            return Some("No 'Refer'/'Referer'");
        }

        //Host
        let host_option = headers.get("Host");
        if let Some(host) = host_option.and_then(|ua| ua.to_str().ok()) {
            if host != "gacha-plus.com" && host != "gachaplus.femlol.hu" {
                return Some("'Host' - 'gacha-plus.com'");
            }
        } else {
            return Some("No 'Host'");
        }
    }

    None
}
