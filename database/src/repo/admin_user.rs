use sea_orm::{EntityTrait, TryIntoModel};
use sea_orm::ActiveValue::Set;
use tracing::info;

use crate::db::error::DbErr;
use crate::db::REPOSITORY;
use crate::entity::admin_user;
use crate::entity::admin_user::ActiveModel;
use crate::entity::prelude::AdminUser;

pub async fn get_user(id: &str) -> Result<admin_user::Model, DbErr> {
    let conn = &REPOSITORY.get().expect("").sea_orm;
    let option = AdminUser::find_by_id(id).one(conn).await?;
    let mut model:ActiveModel = option.unwrap().into();
    info!("result:{:?}",model.mail_address);
    model.mail_address = Set("ward_code".to_owned());
    // let result = model.update(conn).await?;
    // info!("result:{:?}",result.mail_address);
    Ok(model.try_into_model().unwrap())
}