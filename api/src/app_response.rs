use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct GlobalResponse<T: Serialize> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

}

#[allow(dead_code)]
pub fn new<T: Serialize>(code: u16, msg: &str, data: Option<T>) -> Json<GlobalResponse<T>> {
    Json(GlobalResponse { code, message: msg.to_string(), data })
}

#[allow(dead_code)]
pub fn success<T: Serialize>(data: T) -> Json<GlobalResponse<T>> {
    new(0, "OK", Some(data))
}

#[allow(dead_code)]
pub fn success_empty<T: Serialize>() -> Json<GlobalResponse<T>> {
    new(0, "OK", None)
}

#[allow(dead_code)]
pub fn fail<T: Serialize>(msg: &str) -> Json<GlobalResponse<T>> {
    fail_with_code(50000, msg)
}

#[allow(dead_code)]
pub fn fail_with_code<T: Serialize>(code: u16, msg: &str) -> Json<GlobalResponse<T>> {
    new(code, msg, None)
}


