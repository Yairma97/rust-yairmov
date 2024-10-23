use database::{create_user, get_user};
use database::entity::admin_user;

use crate::DomainError;

#[derive(Clone, Debug)]
pub struct UsersManager;

impl UsersManager {
    pub async fn get_user(&self, id: &str) -> Result<admin_user::Model, DomainError> {
        Ok(get_user(id).await?)
    }

    pub async fn create_user(&self, user_name: &str, password: &str) -> Result<i32, DomainError> {
        Ok(create_user(user_name,password).await?)
    }
}