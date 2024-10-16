use axum::Router;
use axum::routing::{get};
use crate::AppState;
use crate::login::login;
use crate::me::me;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(me))
        .route("/get", get(login))
}