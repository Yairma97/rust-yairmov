use crate::proto::hello::greeter_server::GreeterServer;
use crate::service::rpc::hello::MyGreeter;
use tonic_web::CorsGrpcWeb;

pub mod hello;

pub fn rpc_greeter() -> CorsGrpcWeb<GreeterServer<MyGreeter>> {
    tonic_web::enable(GreeterServer::new(MyGreeter {}))
}
