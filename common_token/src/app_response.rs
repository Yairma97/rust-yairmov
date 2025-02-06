use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

}

#[allow(dead_code)]
pub fn new<T>(code: u16, msg: &str, data: Option<T>) -> Json<Response<T>> {
    Json(Response { code, message: msg.to_string(), data })
}

#[allow(dead_code)]
pub fn success<T>(data: T) -> Json<Response<T>> {
    new(0, "OK", Some(data))
}

#[allow(dead_code)]
pub fn success_empty<T>() -> Json<Response<T>> {
    new(0, "OK", None)
}

#[allow(dead_code)]
pub fn fail<T>(msg: &str) -> Json<Response<T>> {
    fail_with_code(50000, msg)
}

#[allow(dead_code)]
pub fn fail_with_code<T>(code: u16, msg: &str) -> Json<Response<T>> {
    new(code, msg, None)
}


