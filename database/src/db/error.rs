use thiserror::Error;
#[derive(Error, Debug)]
pub enum  DbErr {
    #[error("DBErr")]
    DbErr(#[from] sea_orm::error::DbErr),
}