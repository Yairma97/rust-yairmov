use common_token::app_error;
pub use users::*;
pub mod routes;
pub mod users;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, app_error::AppError>;
