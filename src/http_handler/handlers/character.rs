use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use serde::Deserialize;

use crate::http_handler::{response_manager::ResponseManager, AppState};
use crate::{character_code::CharacterCode, gachaplus_database::oc_table::Oc};

#[derive(Deserialize)]
pub struct OcGetParam {
    accountx: String,
}
#[derive(Deserialize)]
pub struct OcAddParam {
    checknum: i32,
    mycode: String,
    accountx: String,
    secretid: String,
}

#[axum::debug_handler]
pub async fn get_oc(
    State(app_state): State<Arc<AppState>>,
    Form(param): Form<OcGetParam>,
) -> Response {
    let accountx = param.accountx.to_uppercase().trim().to_owned();
    if !is_id(&accountx) || accountx.len() != 7 {
        return (
            StatusCode::BAD_REQUEST,
            format!("Invalid `accountx`: {accountx}"),
        )
            .into_response();
    }

    //free ocs cache
    {
        let reader = app_state.oc_chache.read().await;
        let free_oc_res = reader.iter().find(|item| item.accountx == accountx);
        if let Some(free_oc) = free_oc_res {
            return ResponseManager::new_ok()
                .add("xmycode", &free_oc.mycode)
                .into_response();
        }
    }

    //ocs table
    let oc_result = app_state.database.oc_table.get_oc(&accountx).await;

    match oc_result {
        Ok(oc) => match CharacterCode::new_from_code(&oc.mycode) {
            Ok(character) => ResponseManager::new_ok()
                .add("xmycode", &character.to_code())
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid `mycode`: {err}, code: {}", oc.mycode),
            )
                .into_response(),
        },
        Err(_) => (StatusCode::BAD_REQUEST, "No result").into_response(),
    }
}

#[axum::debug_handler]
pub async fn add_oc(
    State(app_state): State<Arc<AppState>>,
    Form(param): Form<OcAddParam>,
) -> Response {
    //checking things

    if param.checknum != 2 {
        return (StatusCode::BAD_REQUEST, "Invalid `checksum`").into_response();
    }
    let character_result = CharacterCode::new_from_code(&param.mycode);
    if let Err(err) = character_result {
        return (
            StatusCode::BAD_REQUEST,
            format!("Invalid `mycode`: {err}, input: {}", param.mycode),
        )
            .into_response();
    }

    let accountx = param.accountx.to_uppercase().trim().to_owned();
    let secretid = param.secretid.to_uppercase().trim().to_owned();

    if !is_id(&accountx) || accountx.len() != 7 {
        return (StatusCode::BAD_REQUEST, "Invalid `accountx`").into_response();
    }

    if !is_id(&secretid) || secretid.len() != 9 {
        return (StatusCode::BAD_REQUEST, "Invalid `secretid`").into_response();
    }

    //upload

    let oc = Oc::new(accountx, secretid, character_result.unwrap().to_code());

    let res = app_state
        .database
        .oc_table
        .insert_or_update_oc(oc.clone())
        .await;

    if res.is_ok() {
        ResponseManager::new_ok().into_response()
    } else {
        (
            StatusCode::BAD_REQUEST,
            format!("Upload error: {:?}", res.unwrap_err()),
        )
            .into_response()
    }
}

fn is_id(id: &str) -> bool {
    id.chars()
        .all(|c| c.is_numeric() || c.is_ascii_uppercase() || c == '#' || c == '$')
}
