use config::{Config, File};
use nacos_sdk::api::config::ConfigServiceBuilder;
use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, ConfigService};
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{
    NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder, ServiceInstance,
};
use nacos_sdk::api::props::ClientProps;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::sync::Arc;

pub static CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::new();
#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
    pub global: IgnoreUrls,
    pub app: AppInnerConfig,
    pub nacos: NacosInnerConfig,
}
#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct IgnoreUrls {
    pub ignores: Vec<String>,
}

#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppInnerConfig {
    pub name: String,
    pub ip: String,
    pub port: String,
    #[serde(skip)]
    pub addr: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub work_id: String,
}

#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct NacosInnerConfig {
    pub server_addr: String,
    pub nacos_config: NacosConfig,
}

#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct NacosConfig {
    pub config_id: String,
    pub namespace: String,
    pub username: String,
    pub password: String,
}

#[allow(unused)]
impl AppConfig {
    pub async fn init(file_path: &str) {
        let settings = Config::builder()
            .add_source(File::with_name(file_path))
            .build()
            .unwrap();
        let mut config = settings.try_deserialize::<AppConfig>().unwrap();
        let addr = format!("{}:{}", config.app.ip.clone(), config.app.port.clone());
        config.app.addr = addr;

        println!("{:#?}", config);
        //nacos
        let client_props = ClientProps::new()
            .server_addr(config.nacos.server_addr.clone())
            // .remote_grpc_port(9838)
            .app_name(config.app.name.clone())
            // Attention! "public" is "", it is recommended to customize the namespace with clear meaning.
            .namespace(config.nacos.nacos_config.namespace.clone())
            .auth_username(config.nacos.nacos_config.username.clone())
            .auth_password(config.nacos.nacos_config.password.clone());

        // ----------  Config  -------------
        let config_service = ConfigServiceBuilder::new(client_props.clone())
            .enable_auth_plugin_http()
            .build()
            .expect("Failed to build config service");
        let config_resp = config_service
            .get_config(
                config.nacos.nacos_config.config_id.clone(),
                constants::DEFAULT_GROUP.to_string(),
            )
            .await;
        match config_resp {
            Ok(config_resp) => tracing::info!("get the config",),
            Err(err) => tracing::error!("get the config {:?}", err),
        }
        let _listen = config_service
            .add_listener(
                config.nacos.nacos_config.config_id.clone(),
                constants::DEFAULT_GROUP.to_string(),
                std::sync::Arc::new(SimpleConfigChangeListener {}),
            )
            .await;
        match _listen {
            Ok(_) => tracing::info!("listening the config success"),
            Err(err) => tracing::error!("listen config error {:?}", err),
        }
        // ----------  Naming  -------------
        let naming_service = NamingServiceBuilder::new(client_props)
            .enable_auth_plugin_http()
            .build()
            .expect("Failed to build naming service");
        let listener = std::sync::Arc::new(SimpleInstanceChangeListener);
        let _subscribe_ret = naming_service
            .subscribe(
                config.app.name.clone(),
                Some(constants::DEFAULT_GROUP.to_string()),
                Vec::default(),
                listener,
            )
            .await;

        let service_instance = ServiceInstance {
            ip: config.app.ip.clone(),
            port: config.app.port.clone().parse().unwrap(),
            ..Default::default()
        };
        let _register_instance_ret = naming_service
            .batch_register_instance(
                config.app.name.clone(),
                Some(constants::DEFAULT_GROUP.to_string()),
                vec![service_instance],
            )
            .await;

        CONFIG.set(Arc::from(config));
    }
}
struct SimpleConfigChangeListener;

impl ConfigChangeListener for SimpleConfigChangeListener {
    fn notify(&self, config_resp: ConfigResponse) {
        tracing::info!("listen the config={}", config_resp);
    }
}

pub struct SimpleInstanceChangeListener;

impl NamingEventListener for SimpleInstanceChangeListener {
    fn event(&self, event: std::sync::Arc<NamingChangeEvent>) {
        tracing::info!("subscriber notify: {:?}", event);
    }
}
