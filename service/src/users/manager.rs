use database::entity::admin_user;

use crate::DomainError;

pub trait UsersManager {
    async fn get_user(
        &self,
        id: &str,
    ) -> Result<admin_user::Model, DomainError>;
}
