use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

static TOKEN_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("jwt secret must set"));

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    random: String,
    user_account: String,
    user_id: String,
    org_code: String,
    role_ids: String,
    user_name: String,
}

#[allow(dead_code)]
pub fn encode_token(user: Claims) -> String {
    encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret(TOKEN_SECRET.as_ref()),
    ).unwrap()
}

#[allow(dead_code)]
pub fn decode_token(token: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&[""]);
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(TOKEN_SECRET.as_ref()),
        &validation,
    ).map(|token_data| token_data.claims)
}


