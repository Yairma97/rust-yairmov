//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "admin_application_menu")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub application_id: String,
    pub menu_level: i32,
    pub parent_menu_level1: String,
    pub parent_menu_level2: String,
    pub menu_code: Option<String>,
    pub menu_name: String,
    pub menu_url: String,
    pub menu_order: i32,
    pub menu_type: i32,
    pub enable: i32,
    pub create_time: DateTime,
    pub create_user_id: String,
    pub create_user_name: String,
    pub update_time: DateTime,
    pub update_user_id: String,
    pub update_user_name: String,
    pub project_id: String,
    pub deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
