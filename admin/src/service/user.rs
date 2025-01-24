use crate::database::user::{create_user, get_user};
use crate::model::entity::admin_user;
use common_token::app_error::AppError;

#[derive(Clone, Debug)]
pub struct UsersService;

impl UsersService {
    pub async fn get_user(&self, id: &str) -> Result<admin_user::Model, AppError> {
        get_user(id).await
    }

    pub async fn create_user(&self, user_name: &str, password: &str) -> Result<i32, AppError> {
        create_user(user_name, password).await
    }
}
