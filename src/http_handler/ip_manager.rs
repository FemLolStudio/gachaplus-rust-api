use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use axum::http::HeaderMap;
use md5::{Digest, Md5};

pub fn get_user_ip(addr: SocketAddr, headers: HeaderMap) -> String {
    //let headers = request.headers().unwrap();

    let add = &addr.ip().to_string();
    let ipaddress = if let Some(value) = headers.get("cf-connecting-ip") {
        value.to_str().unwrap_or(add)
    } else if let Some(value) = headers.get("client-ip") {
        value.to_str().unwrap_or(add)
    } else if let Some(value) = headers.get("x-forwarded-for") {
        value.to_str().unwrap_or(add)
    } else if let Some(value) = headers.get("x-forwarded") {
        value.to_str().unwrap_or(add)
    } else if let Some(value) = headers.get("x-cluster-client-ip") {
        value.to_str().unwrap_or(add)
    } else if let Some(value) = headers.get("forwarded-for") {
        value.to_str().unwrap_or(add)
    } else if let Some(value) = headers.get("forwarded") {
        value.to_str().unwrap_or(add)
    } else {
        add
    };
    ipaddress.to_owned()
}

/// Creating `hash` from `IP` *(because data privacy reasons)* and converting it into a `u32`.
///
/// *(It's only need for statistics, so the data loss doesn't matter.)*
pub fn ip_to_long(ip: &str) -> Option<u32> {
    match IpAddr::from_str(ip.trim()) {
        Ok(_) => {
            let mut hasher = Md5::new();
            hasher.update(ip.trim());
            let result = hasher.finalize();

            let bytes = result[0..4].try_into().ok()?;
            Some(u32::from_be_bytes(bytes))
        }
        Err(_) => None,
    }
}
