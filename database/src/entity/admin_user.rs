//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "admin_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_account: String,
    pub password: String,
    pub user_name: String,
    pub first_login: i32,
    pub gender: Option<String>,
    pub org_code: Option<String>,
    pub dept_code: String,
    pub ward_code: String,
    pub telephone: String,
    pub mail_address: String,
    pub enable: i32,
    pub sync_type: i32,
    pub create_time: DateTime,
    pub create_user_id: String,
    pub create_user_name: String,
    pub update_time: DateTime,
    pub update_user_id: String,
    pub update_user_name: String,
    pub deleted: i32,
    pub user_type: i32,
    #[sea_orm(unique)]
    pub api_access_key: Option<String>,
    pub enable_api_access_key_flag: Option<i32>,
    pub api_access_key_create_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
