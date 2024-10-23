use axum::Router;
use axum::routing::post;

use crate::AppState;
use crate::create_user::create_user;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_user))
}