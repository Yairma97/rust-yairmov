use axum::Json;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app_error::AppError;

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct LoginForm {
    #[validate(length(min = 2, max = 20, code = "login-valid-username"))]
    username: String,
    #[validate(length(min = 8, max = 32, code = "login-valid-password"))]
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[tracing::instrument()]
pub async fn login(
) -> Result<Json<LoginResponse>, AppError> {
    unimplemented!()
}
