use thiserror::Error;

use database::db::error::DbErr;

#[derive(Error, Debug)]
pub enum  DomainError {
    #[error("ServiceError {0}")]
    DbErr(#[from] DbErr),
}
