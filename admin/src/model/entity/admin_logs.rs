//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "admin_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_code: String,
    pub org_name: String,
    pub project_id: String,
    pub project_name: String,
    pub application_id: String,
    pub application_name: String,
    pub menu_name: String,
    pub menu_id: String,
    pub operation: String,
    pub operation_content: String,
    pub success: i32,
    pub ip: String,
    pub client: String,
    pub create_time: DateTime,
    pub create_user_id: String,
    pub create_user_name: String,
    pub update_time: DateTime,
    pub update_user_id: String,
    pub update_user_name: String,
    pub deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
