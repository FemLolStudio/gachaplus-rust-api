use std::sync::{Arc, LazyLock};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use serde::Deserialize;

use crate::{
    gachaplus_database::tranfer_datas_table::TransferDatas,
    http_handler::{response_manager::ResponseManager, AppState},
};

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct GetTransferInput {
    pub systemCall: String,
    pub accountx: u32,
}

static EXTRA_NAMES: LazyLock<String> = LazyLock::new(|| {
    let mut extra_names = Vec::new();
    for i in 0..450 {
        extra_names.push(format!("extraname{i}"));
    }
    extra_names.join("|")
});
static EXTRA_SLOTS: LazyLock<String> = LazyLock::new(|| {
    let mut extra_names = Vec::new();
    for i in 0..450 {
        extra_names.push(format!("extraslot{i}"));
    }
    extra_names.join("|")
});

#[axum::debug_handler]
pub async fn get_transfer_datas(
    State(app_state): State<Arc<AppState>>,
    Form(input): Form<GetTransferInput>,
) -> Response {
    if input.systemCall != "checkLogin" {
        return (StatusCode::BAD_REQUEST, "Invalid value of `systemCall`").into_response();
    }
    if input.accountx < 100_000_000 || input.accountx > 999_999_999 {
        return (StatusCode::BAD_REQUEST, "Invalid value of `accountx`").into_response();
    }
    let res = app_state
        .database
        .tranfer_datas_table
        .get(input.accountx)
        .await;
    if let Ok(row) = res {
        if let Some(error) = row.data.is_invalid() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Transfer data is not valid: {error}"),
            )
                .into_response();
        }

        if app_state
            .database
            .tranfer_datas_table
            .update(input.accountx)
            .await
            .is_err()
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error at updating the database",
            )
                .into_response();
        }

        ResponseManager::new_ok()
            .add("accountx", &row.accountx.to_string())
            .add("datastring1", &row.data.datastring1)
            .add("datastring2", &row.data.datastring2)
            .add("datastring3", &row.data.datastring3)
            .add("datastring4", &row.data.datastring4)
            .add("datastring5", &row.data.datastring5)
            .add("datastring6", &row.data.datastring6)
            .add("datastring7", &row.data.datastring7)
            .add("datastring8", &row.data.datastring8)
            .add("datastring9", &row.data.datastring9)
            .add("datastring10", &row.data.datastring10)
            .add("datastring11", &row.data.datastring11)
            .add("datastring12", &row.data.datastring12)
            .add("datastring13", &row.data.datastring13)
            .add("datastring14", &row.data.datastring14)
            .add("datastring15", &row.data.datastring15)
            .add("datastring16", &row.data.datastring16)
            .add("datastring17", &row.data.datastring17)
            .add("datastring18", &row.data.datastring18)
            .add("datastring19", &row.data.datastring19)
            .add("datastring20", &row.data.datastring20)
            .add(
                "datastring21",
                &row.data.datastring21.unwrap_or(EXTRA_NAMES.to_string()),
            )
            .add(
                "datastring22",
                &row.data.datastring22.unwrap_or(EXTRA_SLOTS.to_string()),
            )
            .into_response()
    } else {
        (
            StatusCode::BAD_REQUEST,
            format!("Database error: {:?}", res.unwrap_err()),
        )
            .into_response()
    }
}

#[axum::debug_handler]
pub async fn add_transfer_datas(
    State(app_state): State<Arc<AppState>>,
    Form(input): Form<TransferDatas>,
) -> Response {
    if let Some(error) = input.is_invalid() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Input data is invalid {error}"),
        )
            .into_response();
    }
    match app_state
        .database
        .tranfer_datas_table
        .insert_or_update(input)
        .await
    {
        Ok(_) => ResponseManager::new_ok()
            .add("msg", "Uploaded successfully")
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Upload error: {error}"),
        )
            .into_response(),
    }
}
