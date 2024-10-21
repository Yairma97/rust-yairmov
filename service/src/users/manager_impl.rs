use database::entity::admin_user;
use database::get_user;

use crate::{DomainError, UsersManager};

#[derive(Clone, Debug)]
pub struct UsersManagerImpl;

impl UsersManager for UsersManagerImpl {
    async fn get_user(&self, id: &str) -> Result<admin_user::Model, DomainError> {
        Ok(get_user(id).await?)
    }
}