use std::future::Future;
use crate::DomainError;

use crate::models::{RegistryUsers, Users};

pub trait UsersManager {
    fn add_user(
        &self,
        reg_user: RegistryUsers,
    ) -> impl Future<Output = Result<Users, DomainError>> + Send;

}
