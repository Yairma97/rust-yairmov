
use serde::Serialize;
use vars::ID;

#[derive(Serialize, Debug, Clone)]
pub struct Users {
    pub id: ID,
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}
#[derive(Serialize, Debug, Clone)]
pub struct RegistryUsers {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}
