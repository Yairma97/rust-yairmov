use crate::{app_request::JwtAuth, AppState};
use axum::{extract::State, Json};
use axum::response::IntoResponse;
use serde::Serialize;
use tracing::info;
use crate::app_error::AppError;
use crate::app_response::{GlobalResponse, success};

#[derive(Serialize)]
pub struct MeResponse {
    pub username: String,
    pub roles: Vec<String>,
}

#[tracing::instrument()]
pub async fn me(
    JwtAuth(user_info): JwtAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    info!("state:{:?}",state);
    Ok(success(MeResponse {
        username: "dd".to_string(),
        roles: vec!["dd".to_string()],
    }))
}
