use sea_orm::{EntityTrait};
use crate::db::error::DbErr;
use crate::db::REPOSITORY;
use crate::entity::admin_user;
use crate::entity::prelude::AdminUser;


pub async fn get_user(id: &str) -> Result<admin_user::Model, DbErr> {
    let x = &REPOSITORY.get().expect("").sea_orm;
    let option = AdminUser::find_by_id(id).one(x).await?;
    let model = option.unwrap();
    Ok(model)
}