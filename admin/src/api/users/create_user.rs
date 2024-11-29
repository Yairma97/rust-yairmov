use crate::service::UsersManager;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use common_token::app_error::AppError;
use common_token::app_request::JwtAuth;
use common_token::app_response::success;
use common_token::app_state::AppState;
use serde::{Deserialize, Serialize};
use tracing::info;

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
    info!("{:#?}",user_info);
    info!("{:#?}",state);
    let sucess = UsersManager.create_user(params.user_name.as_str(), params.password.as_str()).await?;
    Ok(success(sucess))
}
