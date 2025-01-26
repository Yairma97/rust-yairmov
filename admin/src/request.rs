use crate::config::CONFIG;
use crate::error::AppError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{
    async_trait,
    extract::{FromRequest, Path, Query, Request},
    Json,
};
use common_token::jwt::{decode_token, Claims};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use validator::{Validate, ValidationErrors};
use wax::Pattern;


#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::from(e))?;
        value.validate().map_err(|e| {
            let ves = to_new_validation_errors(e);
            AppError::from(ves)
        })?;
        Ok(ValidatedQuery(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::from(AppError::from(e)))?;
        value.validate().map_err(|e| {
            let ves = to_new_validation_errors(e);
            AppError::from(ves)
        })?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPath<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedPath<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Path(value) = Path::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::from(AppError::from(e)))?;
        value.validate().map_err(|e| {
            let ves = to_new_validation_errors(e);
            AppError::from(ves)
        })?;
        Ok(ValidatedPath(value))
    }
}

fn to_new_validation_errors(e: ValidationErrors) -> ValidationErrors {
    tracing::debug!("e.field_errors(): {:?}", e.field_errors());
    let mut new_validation_errors = ValidationErrors::new();
    for (field, vec_validation_error) in e.field_errors() {
        for validation_err in vec_validation_error {
            tracing::debug!("validation_err.code: {}", validation_err.code);
            let mut new_validation_error = validation_err.clone();
            new_validation_error.message =
                Some(Cow::from(new_validation_error.code.clone().to_string()));
            new_validation_errors.add(field, new_validation_error);
        }
    }
    tracing::debug!(
        "ves.field_errors(): {:?}",
        new_validation_errors.field_errors()
    );

    new_validation_errors
}

pub struct JwtAuth(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for JwtAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = CONFIG.get().unwrap();
        let vec = config.config.get::<Vec<String>>("global.ignores")?;
        let path = req.uri.path();
        for ignore_url in vec {
            if wax::Glob::new(&ignore_url)?.is_match(path) {
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
