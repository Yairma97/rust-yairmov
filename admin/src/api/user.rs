use crate::error::AppError;
use crate::model::params::user::CreateUserParams;
use crate::request::JwtAuth;
use crate::service::user::UsersService;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use common_token::app_response::success;
use common_token::app_state::AppState;
use std::collections::HashMap;
use tracing::info;

//api
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_user))
        .route("/get", get(get_user))
}

#[tracing::instrument(skip(state))]
pub async fn create_user(
    JwtAuth(user_info):JwtAuth,
    State(state): State<AppState>,
    Json(params):Json<CreateUserParams>
) -> Result<impl IntoResponse, AppError> {
    info!("{:#?}",user_info);
    info!("{:#?}",state);
    let sucess = UsersService.create_user(params.user_name.as_str(), params.password.as_str()).await?;
    Ok(success(sucess))
}

#[tracing::instrument(skip(state,user_info))]
pub async fn get_user(
    JwtAuth(user_info):JwtAuth,
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<impl IntoResponse, AppError> {
    let id = params.get("id").expect("id不能为空");
    let sucess = UsersService.get_user(id).await?;
    Ok(success(sucess))
}
