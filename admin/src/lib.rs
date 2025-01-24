use crate::config::{AppConfig, CONFIG};
use crate::database::Repo;
use common_token::app_state::Context;
use dashmap::DashMap;
use idgenerator_thin::{IdGeneratorOptions, YitIdHelper};
use std::net::SocketAddr;
use std::sync::Arc;

mod api;
mod config;
mod database;
mod extra;
mod model;
mod request;
mod service;

pub type IdHelper = YitIdHelper;
pub async fn start() {
    dotenv::from_path("admin/.env").ok();

    let (_guard_file, _guard_stderr) = extra::init().await;

    AppConfig::init("admin/app.yaml").await;

    Repo::create().await;

    let app_config = CONFIG.get().expect("APPConfig is not set");
    let work_id = &app_config.app.work_id;
    let options = IdGeneratorOptions::new(work_id.parse::<u32>().unwrap());
    YitIdHelper::set_id_generator(options);

    let bind_address = &app_config
        .app
        .addr
        .parse::<SocketAddr>()
        .unwrap();

    let app_state = Arc::new(Context {
        context: DashMap::new(),
    });
    let routes = api::routes(app_state);

    println!("listening on {}", bind_address);

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
