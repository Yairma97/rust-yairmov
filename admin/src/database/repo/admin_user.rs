use crate::database::entity::admin_user;
use crate::database::entity::admin_user::ActiveModel;
use crate::database::entity::prelude::AdminUser;
use crate::database::REPOSITORY;
use common_token::app_error::AppError;
use idgenerator_thin::YitIdHelper;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, TryIntoModel};
use sea_query::{PostgresQueryBuilder, Query};
use tracing::info;

pub async fn get_user(id: &str) -> Result<admin_user::Model, AppError> {
    let conn = &REPOSITORY.get().expect("").sea_orm;
    let option = AdminUser::find_by_id(id).one(conn).await?;
    let mut model: ActiveModel = option.unwrap().into();
    info!("result:{:?}",model.mail_address);
    model.mail_address = Set("ward_code".to_owned());
    // let result = model.update(conn).await?;
    // info!("result:{:?}",result.mail_address);
    Ok(model.try_into_model().unwrap())
}
pub async fn create_user(user_name: &str, password: &str) -> Result<i32, AppError> {
    let conn = &REPOSITORY.get().expect("").sea_orm;
    let admin_user = admin_user::ActiveModel {
        id: Set(YitIdHelper::next_id().to_string()),
        user_account: Set(user_name.to_string()),
        password: Set(password.to_string()),
        user_name: Set(user_name.to_string()),
        ..Default::default()
    };
    let sql = Query::insert()
        .into_table(AdminUser)
        .columns([admin_user::Column::Id])
        .values_panic(["1111".into()])
        .to_string(PostgresQueryBuilder);
    println!("PostgresQueryBuilder {}", sql.as_str(), );
    let model = admin_user.insert(conn).await?;
    print!("model:{:?}", model);
    Ok(1)
}