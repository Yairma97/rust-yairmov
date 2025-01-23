use crate::config::CONFIG;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait};
use common_token::app_error::AppError;
use common_token::jwt::{decode_token, Claims};
use wax::Pattern;

pub struct JwtAuth(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for JwtAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = CONFIG.get().unwrap();
        let vec = config.global.ignores.iter();
        let path = req.uri.path();
        for ignore_url in vec {
            if wax::Glob::new(ignore_url)?.is_match(path) {
                return Ok(Self(Default::default()));
            }
        }
        let headers = req.to_owned().headers;
        let auth = headers
            .get("Authorization")
            .expect("No authorization header");
        let claims = decode_token(auth.to_str()?)?;
        Ok(Self(claims))
    }
}
