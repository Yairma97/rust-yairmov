use crate::database::REPOSITORY;
use crate::error::AppError;
use crate::model::entity::admin_user;
use crate::model::entity::prelude::AdminUser;
use crate::IdHelper;
use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};


#[derive(Clone, Debug, Deserialize, FromRow, Serialize,Default)]
pub struct AdminUserD {
    pub id: Option<String>,
    pub user_account: Option<String>,
    pub password: Option<String>,
    pub user_name: Option<String>,
    pub first_login:Option<String>,
}

pub async fn get_user(id: &str) -> Result<admin_user::Model, AppError> {
    let conn = &REPOSITORY.get().expect("").sea_orm;
    let option = AdminUser::find_by_id(id).one(conn).await?;
    option.ok_or(anyhow!("User not found").into())
}
pub async fn create_user(user_name: &str, password: &str) -> Result<i32, AppError> {
    println!("{}", IdHelper::next_id().to_string());
    // let conn = &REPOSITORY.get().expect("").sea_orm;
    // let admin_user = admin_user::ActiveModel {
    //     id: Set(YitIdHelper::next_id().to_string()),
    //     user_account: Set(user_name.to_string()),
    //     password: Set(password.to_string()),
    //     user_name: Set(user_name.to_string()),
    //     ..Default::default()
    // };
    // let sql = Query::insert()
    //     .into_table(AdminUser)
    //     .columns([admin_user::Column::Id])
    //     .values_panic(["1111".into()])
    //     .to_string(PostgresQueryBuilder);
    // println!("PostgresQueryBuilder {}", sql.as_str(), );
    // let model = admin_user.insert(conn).await?;
    // print!("model:{:?}", model);
    Ok(1)
}