use thiserror::Error;

#[derive(Error, Debug)]
pub enum  DbErr {
    #[error("DBErr {0}")]
    DbErr(#[from] sea_orm::error::DbErr),
}