use thiserror::Error;
use database::db::error::DbErr;
use util::i18n::i18n;

#[derive(Error, Debug)]
#[error("{}", i18n("something-wrong"))]
pub enum  DomainError {
    #[error("ServiceError")]
    DbErr(#[from] DbErr),
}
