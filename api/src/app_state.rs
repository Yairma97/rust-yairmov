use std::{env, net::SocketAddr, sync::Arc};

use dashmap::DashMap;
use idgenerator_thin::{IdGeneratorOptions, YitIdHelper};

use crate::app_config::AppConfig;
use crate::app_routes;

#[derive(Debug)]
pub struct Context {
    pub context: DashMap<String, String>,
}

pub type AppState = Arc<Context>;


pub async fn start() {
    AppConfig::init("app.yaml");
    let app_state = Arc::new(Context { context: DashMap::new() });

    let options = IdGeneratorOptions::new(1);
    YitIdHelper::set_id_generator(options);

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let routes = app_routes::routes(app_state);

    println!("listening on {}", bind_address);

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
