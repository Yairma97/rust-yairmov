use crate::config::{AppConfig, CONFIG};
use crate::database::Repo;
use crate::proto::hello;
use crate::service::rpc::hello::MyGreeter;
use crate::service::Service;
use common_token::app_state::Context;
use dashmap::DashMap;
use idgenerator_thin::{IdGeneratorOptions, YitIdHelper};
use sea_query::ExprTrait;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use tonic::service::Routes;

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
    //db
    Repo::create().await;
    println!("----------------------------");
    //rpc
    Service::init().await.expect("service init error");
    println!("======================");
    let app_config = CONFIG.get().expect("APPConfig is not set");
    let work_id = app_config.config.get_string("app.work_id").unwrap();
    let options = IdGeneratorOptions::new(work_id.parse::<u32>().unwrap());
    YitIdHelper::set_id_generator(options);
    //state
    let app_state = Arc::new(Context {
        context: DashMap::new(),
    });
    //route
    let total = Routes::builder();
    let rest_service = api::routes(app_state);
    //rpc
    let grpc_hello = tonic_web::enable(hello::greeter_server::GreeterServer::new(MyGreeter{}));
    let router = total.routes()
        .add_service(grpc_hello)
        .into_axum_router()
        .merge(rest_service);
    let addr = format!(
        "{}:{}",
        app_config.config.get_string("app.ip").unwrap(),
        app_config.config.get_string("app.port").unwrap()
    );
    let bind_address = addr.parse::<SocketAddr>().unwrap();
    println!("listening on {}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, router)
        .await
        .unwrap();
}
