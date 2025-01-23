use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct CreateUserParams {
    #[serde(rename="username")]
    pub user_name: String,
    pub password: String,
}