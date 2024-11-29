use std::fmt::{Display, Formatter};

use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::header::ToStrError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

use crate::app_response::GlobalResponse;

#[derive(Debug)]
pub struct AppError(pub Response);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.0
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AppError {
    pub(crate) fn forbidden(str: String) -> Self {
        Self::error_response(str, StatusCode::FORBIDDEN)
    }

    pub fn internal_server_error(str: String) -> Self {
        Self::error_response(str, StatusCode::INTERNAL_SERVER_ERROR)
    }
    #[allow(dead_code)]
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
        error!("Unhandled internal error: {}", message);
        Self(
            Json(GlobalResponse::<String> {
                message,
                code: code.as_u16(),
                data: None,
            })
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

impl From<ValidateError> for AppError {
    fn from(e: ValidateError) -> Self {
        match &e {
            ValidateError::InvalidParam(v) => Self::bad_request(v.to_string().replace('\n', " , ")),
            ValidateError::AxumQueryRejection(v) => Self::bad_request(v.to_string()),
            ValidateError::AxumJsonRejection(v) => Self::bad_request(v.to_string()),
            ValidateError::AxumPathRejection(v) => Self::bad_request(v.to_string()),
        }
    }
}

impl From<JWTError> for AppError {
    fn from(e: JWTError) -> Self {
        match &e {
            JWTError::Invalid => Self::unauthorized(e.to_string()),
            JWTError::Missing => Self::forbidden(e.to_string()),
        }
    }
}
impl From<sea_orm::DbErr> for AppError {
    fn from(e: sea_orm::DbErr) -> Self {
        match &e {
            _ => Self::internal_server_error(e.to_string()),
        }
    }
}
impl From<ToStrError> for AppError {
    fn from(e: ToStrError) -> Self {
        match &e {
            ToStrError { .. } => Self::internal_server_error(e.to_string()),
        }
    }
}

impl From<wax::BuildError> for AppError {
    fn from(e: wax::BuildError) -> Self {
        match &e {
            wax::BuildError { .. } => Self::forbidden(e.to_string()),
        }
    }
}
