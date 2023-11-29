use std::{net::SocketAddr, time::Instant};

use axum::{
    body::{Body, Bytes},
    extract::{ConnectInfo, OriginalUri, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use http_body_util::BodyExt;
use hyper::HeaderMap;
use inline_colorization::*;

use crate::http_handler::ip_manager::get_user_ip;

pub async fn log(
    OriginalUri(path): OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,

    request: Request,
    next: Next,
) -> Response {
    //println!("{:?}", headers);
    let now = Instant::now();
    let mut response = next.run(request).await;
    let delay_in_ms = now.elapsed().as_micros() as f64 / 1000f64;
    let ip = get_user_ip(addr, headers);
    if response.status() == StatusCode::OK || response.status() == StatusCode::PERMANENT_REDIRECT {
        println!(
            "{}{}{}\tCode: {}{}{}\tIP: {}{}{}\tDelay: {}{:.3}{} ms\tRequest: {}{}{}",
            color_cyan,
            Utc::now().format("[%H:%M:%S]"),
            color_green,
            color_cyan,
            response.status(),
            color_green,
            color_cyan,
            ip,
            color_green,
            color_cyan,
            delay_in_ms,
            color_green,
            color_cyan,
            path,
            color_white,
        );
    } else {
        // Buffer the response body
        let (parts, body) = response.into_parts();
        let (bytes, body_string_option) = get_body(body).await;

        let mut body_string = body_string_option.unwrap_or(String::from("<none>"));
        if body_string.is_empty() {
            body_string = String::from("<empty>");
        }
        println!(
            "{}{}\tCode: {}\tIP:{}\tDelay: {:.3} ms\tRequest: {}\tBody: {}{}",
            color_yellow,
            Utc::now().format("[%H:%M:%S]"),
            parts.status,
            ip,
            delay_in_ms,
            path,
            body_string,
            color_white,
        );
        if let Some(bytes) = bytes {
            response = Response::from_parts(parts, Body::from(bytes)).into_response();
        } else {
            response = parts.into_response()
        }
    }

    response
}

pub async fn get_body<B>(body: B) -> (Option<Bytes>, Option<String>)
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes_res = body.collect().await;
    if let Ok(bytes) = bytes_res {
        let bytes = bytes.to_bytes();
        if let Ok(body) = std::str::from_utf8(&bytes) {
            let restr = body.to_owned();
            (Some(bytes), Some(restr))
        } else {
            (Some(bytes), None)
        }
    } else {
        (None, None)
    }
}
