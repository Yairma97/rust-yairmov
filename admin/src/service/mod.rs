pub mod rpc;
pub mod user;

use crate::config::CONFIG;
use crate::error::AppError;
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::ServiceInstance;
use std::net::{SocketAddr, ToSocketAddrs};


pub struct Service;
impl Service {
    pub(crate) async fn init() -> Result<(), AppError> {
        // 服务地址
        let addr: SocketAddr = "[::1]:50051".parse().unwrap();

        let app_config = CONFIG.get().expect("APPConfig is not set");
        let naming_service = &app_config.naming_service;

        let instance = ServiceInstance {
            instance_id: Some(app_config.config.get_string("rpc.name")?),
            ip: app_config.config.get_string("rpc.ip")?,
            port: app_config.config.get_int("rpc.port")? as i32,
            service_name: Some(app_config.config.get_string("rpc.name")?),
            ..Default::default()
        };
        let service_name = app_config.config.get_string("rpc.name")?;
        let group_name = Some(constants::DEFAULT_GROUP.to_string());
        naming_service
            .register_instance(service_name, group_name, instance)
            .await?;

        // 启动 gRPC 服务
        println!("Server running on {}", addr);
        // let server = EchoServer {};
        // Server::builder()
        //     .add_service(pb::echo_server::EchoServer::new(server))
        //     .serve("0.0.0.0:50051".to_socket_addrs().unwrap().next().unwrap())
        //     .await
        //     .unwrap();

        Ok(())
    }
}
