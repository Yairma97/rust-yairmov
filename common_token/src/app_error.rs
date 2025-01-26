use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::BoxError;
use std::io::{self, ErrorKind};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("server error{0}")]
    ServerError(BoxError),

    #[error(transparent)]
    ConfigError(#[from]config::ConfigError),
    /// File IO Error
    #[error(transparent)]
    IOError(#[from] io::Error),

    /// Other runtime errors
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),

    #[error(transparent)]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    WaxBuildError(#[from] wax::BuildError),

    #[error(transparent)]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error(transparent)]
    ToStrError(#[from] axum::http::header::ToStrError),

    #[error(transparent)]
    QueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    PathRejection(#[from] axum::extract::rejection::PathRejection),
}

impl AppError {
    /// Failed to read file io
    pub fn from_io(kind: ErrorKind, msg: &str) -> Self {
        AppError::IOError(io::Error::new(kind, msg))
    }
}
/// Contains the return value of AppError
pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::ServerError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::IOError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::OtherError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::JWTError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::WaxBuildError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::ValidationErrors(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::DbErr(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::ToStrError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::QueryRejection(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::JsonRejection(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::PathRejection(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::ConfigError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
        };

        (status, msg).into_response()
    }
}
