use crate::app_response::AppError;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use domain::{DomainError, GetUserError, };
use macros::FromError;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum ValidateError {
    InvalidParam(ValidationErrors),
    AxumQueryRejection(QueryRejection),
    AxumJsonRejection(JsonRejection),
    AxumPathRejection(PathRejection),
}

#[derive(Debug, FromError)]
pub enum LoginError {
    // #[error("{}", i18n("login-password-not-correct"))]
    #[from_error(code = "login-password-not-correct", status = "forbidden")]
    WrongPassword,
    // #[from_error(code = "login-user-not-exist", status = "unauthorized")]
    // UserNotExist,
}

#[derive(FromError, Debug)]
pub enum RegistryError {
    #[from_error(code = "registry-user-exist", status = "forbidden")]
    UserExist,
}

#[derive(FromError, Debug)]
pub enum ChangeUsernameError {
    #[from_error(code = "user-name-exist", status = "forbidden")]
    UsernameExist,
}

#[derive(FromError, Debug)]
pub enum JWTError {
    // #[error("{}", i18n("jwt-missing"))]
    // Missing,
    #[from_error(code = "jwt-invalid", status = "unauthorized")]
    Invalid,
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> AppError {
        Self::internal_server_error(e.to_string())
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


