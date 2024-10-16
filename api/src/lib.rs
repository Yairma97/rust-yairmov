mod app_error;
mod app_request;
mod app_response;
mod app_routes;
pub mod app_state;
pub mod users;

pub use app_state::*;
pub use users::*;
pub type Result<T> = std::result::Result<T, app_error::AppError>;