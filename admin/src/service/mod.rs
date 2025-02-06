pub mod rpc;
pub mod user;

use crate::error::AppError;


pub struct Service;
impl Service {
    pub(crate) async fn init() -> Result<(), AppError> {
        // let app_config = CONFIG.get().expect("APPConfig is not set");
        // let naming_service = &app_config.naming_service;
        //
        // let instance = ServiceInstance {
        //     instance_id: Some(app_config.config.get_string("rpc.name")?),
        //     ip: app_config.config.get_string("rpc.ip")?,
        //     port: app_config.config.get_int("rpc.port")? as i32,
        //     service_name: Some(app_config.config.get_string("rpc.name")?),
        //     ..Default::default()
        // };
        // let service_name = app_config.config.get_string("rpc.name")?;
        // let group_name = Some(constants::DEFAULT_GROUP.to_string());
        // naming_service
        //     .register_instance(service_name, group_name, instance)
        //     .await?;
        //
        // // 启动 gRPC 服务
        // let addr = format!(
        //     "{}:{}",
        //     app_config.config.get_string("app.ip")?,
        //     app_config.config.get_string("app.port")?,
        // );
        // println!("Server running on {}", addr);
        Ok(())
    }
}
