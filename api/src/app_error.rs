use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::header::ToStrError;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

use service::{DomainError, GetUserError};

use crate::app_response::GlobalResponse;

pub struct AppError(pub Response);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.0
    }
}

impl AppError {
    pub(crate) fn forbidden(str: String) -> Self {
        Self::error_response(str, StatusCode::FORBIDDEN)
    }

    pub(crate) fn internal_server_error(str: String) -> Self {
        Self::error_response(str, StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub(crate) fn not_found(str: String) -> Self {
        Self::error_response(str, StatusCode::NOT_FOUND)
    }

    pub(crate) fn unauthorized(str: String) -> Self {
        Self::error_response(str, StatusCode::UNAUTHORIZED)
    }

    pub(crate) fn bad_request(str: String) -> Self {
        Self::error_response(str, StatusCode::BAD_REQUEST)
    }

    pub(crate) fn error_response(message: String, code: StatusCode) -> Self {
        Self(
            (
                code,
                Json(json!(GlobalResponse::<String> {
                    message,
                    code: code.as_u16(),
                    data: None
                })),
            )
                .into_response(),
        )
    }
}

#[derive(Debug)]
pub enum ValidateError {
    InvalidParam(ValidationErrors),
    AxumQueryRejection(QueryRejection),
    AxumJsonRejection(JsonRejection),
    AxumPathRejection(PathRejection),
}

#[derive(Error, Debug)]
pub enum JWTError {
    #[error("{}", "验证失败，请重新登录")]
    Missing,
    #[error("{}", "未认证")]
    Invalid,
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> AppError {
        match &e {
            DomainError::DbErr(_) => {Self::internal_server_error(e.to_string())}
        }
    }
}

impl From<ValidateError> for AppError {
    fn from(e: ValidateError) -> Self {
        match &e {
            ValidateError::InvalidParam(v) => {
                Self::bad_request(v.to_string().replace('\n', " , "))
            }
            ValidateError::AxumQueryRejection(v) => {
                Self::bad_request(v.to_string())
            }
            ValidateError::AxumJsonRejection(v) => {
                Self::bad_request(v.to_string())
            }
            ValidateError::AxumPathRejection(v) => {
                Self::bad_request(v.to_string())
            }
        }
    }
}

impl From<GetUserError> for AppError {
    fn from(e: GetUserError) -> Self {
        match &e {
            GetUserError::NotFound { .. } => Self::not_found(e.to_string()),
            GetUserError::PasswordNotCorrect { .. } => {
                Self::forbidden(e.to_string())
            }
            GetUserError::DomainError(_) => {
                Self::internal_server_error(e.to_string())
            }
        }
    }
}

impl From<JWTError> for AppError {
    fn from(e: JWTError) -> Self {
        match &e {
            JWTError::Invalid => Self::unauthorized(e.to_string()),
            JWTError::Missing => Self::forbidden(e.to_string())
        }
    }
}

impl From<ToStrError> for AppError {
    fn from(e: ToStrError) -> Self {
        match &e {
            ToStrError { .. } => Self::internal_server_error(e.to_string())
        }
    }
}
impl From<wax::BuildError> for AppError {
    fn from(e: wax::BuildError) -> Self {
        match &e {
            wax::BuildError { .. } => Self::forbidden(e.to_string())
        }
    }
}
