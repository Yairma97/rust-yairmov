use crate::app_routes;
use domain::{
    UsersManagerImpl,
};
use std::{env, net::SocketAddr, sync::Arc};

#[derive(Clone, Debug)]
pub struct AppStateRaw {
    pub users_manager: UsersManagerImpl,
}

pub type AppState = Arc<AppStateRaw>;

pub async fn start() {

    let users_manager = UsersManagerImpl;

    let app_state = Arc::new(AppStateRaw {
        users_manager,
    });
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
