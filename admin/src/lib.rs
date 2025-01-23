use crate::database::Repo;
use common_token::app_state::Context;
use dashmap::DashMap;
use idgenerator_thin::{IdGeneratorOptions, YitIdHelper};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::config::AppConfig;

mod api;
mod database;
mod extra;
mod service;
mod model;
mod config;
mod request;

pub async fn start() {
    dotenv::from_path("admin/.env").ok();

    let (_guard_file, _guard_stderr) = extra::init().await;

    Repo::create().await;

    AppConfig::init("admin/app.yaml");
    let app_state = Arc::new(Context {
        context: DashMap::new(),
    });

    let options = IdGeneratorOptions::new(1);
    YitIdHelper::set_id_generator(options);

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let routes = api::routes(app_state);

    println!("listening on {}", bind_address);

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
