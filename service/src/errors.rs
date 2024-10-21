use thiserror::Error;

use database::db::error::DbErr;

#[derive(Error, Debug)]
#[error("{}", i18n("something-wrong"))]
pub enum  DomainError {
    #[error("ServiceError")]
    DbErr(#[from] DbErr),
}
