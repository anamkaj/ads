use crate::{direct::db::get_statistics::get_statistics, server::server::AppState};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::Instant;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct ReqParam {
    #[validate(length(min = 3))]
    pub client_login: String,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

pub async fn stat_client(
    State(pool): State<Arc<AppState>>,
    param: Query<ReqParam>,
) -> impl IntoResponse {
    let start_time_request: Instant = Instant::now();
    let valid_param = &param.validate();

    match &valid_param {
        Ok(_) => {}
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "status": "error",
                    "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                    "err": err.to_string(),
                })),
            );
        }
    }

    let resp = match get_statistics(pool.db.clone(), &param).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "err": err.to_string(),
            })),
        ),
    };

    resp
}
