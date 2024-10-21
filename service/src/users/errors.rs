use thiserror::Error;

use crate::DomainError;

#[derive(Error, Debug)]
pub enum GetUserError {
    #[error("{}", "user-not-found")]
    NotFound { username: String },
    #[error("{}", "user-password-not-correct")]
    PasswordNotCorrect { username: String },
    #[error("{}", "something-wrong")]
    DomainError(#[from] DomainError),
}
