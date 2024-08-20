use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use tokio::time::Instant;
use crate::{direct::models::client_list::ClientList, server::server::AppState};

pub async fn start_parse_client_list(State(pool): State<Arc<AppState>>) -> impl IntoResponse {
    let start_time_request: Instant = Instant::now();

    let resp = match ClientList::get_client_list(pool.db.clone()).await {
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
