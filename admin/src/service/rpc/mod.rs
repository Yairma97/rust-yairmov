use crate::proto::hello::greeter_server::GreeterServer;
use crate::service::rpc::hello::MyGreeter;
use axum::Router;
use tonic::service::Routes;

pub mod hello;

pub fn rpc_greeter() -> Router {
    Routes::builder().routes()
        .add_service(tonic_web::enable(GreeterServer::new(MyGreeter {})))
        .into_axum_router()

}
