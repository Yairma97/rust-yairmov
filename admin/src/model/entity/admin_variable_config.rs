//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "admin_variable_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub application_id: Option<String>,
    pub project_id: Option<String>,
    pub variable_key: Option<String>,
    pub variable_value: Option<String>,
    pub edit_flag: Option<String>,
    pub create_time: DateTime,
    pub create_user_id: String,
    pub create_user_name: String,
    pub update_time: DateTime,
    pub update_user_id: String,
    pub update_user_name: String,
    pub deleted: i32,
    pub variable_desc: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
