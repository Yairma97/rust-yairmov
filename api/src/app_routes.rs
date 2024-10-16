use std::any::Any;
use crate::{
    AppState,
};
use axum::{
    body::{Body, Bytes},
    error_handling::HandleErrorLayer,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Json, Router,
};
use http_body_util::BodyExt;
use serde_json::json;
use std::time::Duration;
use serde::Serialize;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use crate::app_error::AppError;
use crate::app_response::{GlobalResponse, new, success};
use crate::me::MeResponse;

pub fn routes(state: AppState) -> Router {
    // don't change layer order, or errors happen...
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(30))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(print_request_response));

    Router::new()
        // users
        .nest("/users", crate::users::route::router())
        .layer(middleware_stack.into_inner())
        .with_state(state)
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(
    direction: &str,
    body: B,
) -> Result<Bytes, (StatusCode, String)>
    where
        B: axum::body::HttpBody<Data=Bytes>,
        B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        Ok(StatusCode::REQUEST_TIMEOUT)
    } else {
        Err(AppError::internal_server_error(format!("Unhandled internal error: {}",error)))
    }
}
