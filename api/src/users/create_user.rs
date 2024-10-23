use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use service::UsersManager;

use crate::app_error::AppError;
use crate::app_request::JwtAuth;
use crate::app_response::success;
use crate::AppState;

#[derive(Serialize,Debug,Deserialize)]
pub struct CreateUserParams {
    #[serde(rename="username")]
    pub user_name: String,
    pub password: String,
}

#[tracing::instrument(skip(state))]
pub async fn create_user(
    JwtAuth(user_info):JwtAuth,
    State(state): State<AppState>,
    Json(params):Json<CreateUserParams>
) -> Result<impl IntoResponse, AppError> {
    let sucess = UsersManager.create_user(params.user_name.as_str(), params.password.as_str()).await?;

    Ok(success(sucess))
}
