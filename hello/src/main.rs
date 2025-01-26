mod proto;

use crate::proto::hello::greeter_client::GreeterClient;
use crate::proto::hello::HelloRequest;
use nacos_sdk::api::naming::NamingService;
use nacos_sdk::api::naming::{NamingEventListener, NamingServiceBuilder};
use nacos_sdk::api::props::ClientProps;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //todo 从 nacos 拿到了之后就可以 set 到 config 里面

    // 从 Nacos 获取服务地址
    let client_props = ClientProps::new()
        .server_addr("127.0.0.1:8848")
        .app_name("rust-client")
        .namespace("")
        .auth_username("nacos")
        .auth_password("nacos");;

    let naming_service = NamingServiceBuilder::new(client_props)
        .enable_auth_plugin_http()
        .build()
        .expect("Failed to build naming service");


    let instances = naming_service
        .select_instances("rust-admin-rpc".to_string(), Some("DEFAULT_GROUP".to_string()), vec![], false, true) // 获取健康的实例
        .await?;
    println!("instances: {:?}", instances);
    // 选择一个可用的服务地址
    let instance = instances.first().ok_or("No service available")?;
    let addr = format!("http://{}:{}", instance.ip, instance.port);

    // 创建 gRPC 客户端
    let channel = Channel::from_shared(addr)?.connect().await?;
    let mut client = GreeterClient::new(channel);

    // 调用 gRPC 方法
    let request = tonic::Request::new(HelloRequest {
        name: "World".into(),
    });

    let response = client.say_hello(request).await?;
    println!("Response: {:?}", response);
    Ok(())
}
