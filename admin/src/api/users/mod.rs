use crate::api::create_user::create_user;
use axum::routing::post;
use axum::Router;
use common_token::app_state::AppState;

pub mod create_user;


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_user))
}