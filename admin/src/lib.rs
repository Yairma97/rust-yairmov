use crate::config::{AppConfig, CONFIG};
use crate::database::Repo;
use crate::service::rpc::rpc_greeter;
use crate::service::Service;
use common_token::app_state::Context;
use dashmap::DashMap;
use idgenerator_thin::{IdGeneratorOptions, YitIdHelper};
use std::net::SocketAddr;
use std::sync::Arc;
mod api;
mod config;
mod database;
mod error;
mod extra;
mod model;
mod proto;
mod request;
mod service;

pub type IdHelper = YitIdHelper;
pub async fn start() {
    //env
    dotenv::from_path("admin/.env").ok();
    //log
    let (_guard_file, _guard_stderr) = extra::init().await;
    //config
    AppConfig::init("admin/app.yaml")
        .await
        .expect("config init error");
    println!("-------------config---------------");
    //db
    Repo::create().await;
    println!("--------------db--------------");
    //rpc
    Service::init().await.expect("service init error");
    println!("--------------rpc--------------");
    let app_config = CONFIG.get().expect("APPConfig is not set");
    let work_id = app_config.config.get_string("app.work_id").unwrap();
    let options = IdGeneratorOptions::new(work_id.parse::<u32>().unwrap());
    YitIdHelper::set_id_generator(options);
    //state
    let app_state = Arc::new(Context {
        context: DashMap::new(),
    });
    //route
    let rest_service = api::routes(app_state);
    //rpc
    let rpc = rpc_greeter();
    let router = rest_service.merge(rpc);
    let addr = format!(
        "{}:{}",
        app_config.config.get_string("app.ip").unwrap(),
        app_config.config.get_string("app.port").unwrap()
    );
    let bind_address = addr.parse::<SocketAddr>().unwrap();
    println!("listening on {}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
