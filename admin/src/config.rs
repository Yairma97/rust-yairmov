use crate::error::AppError;
use config::{Config, File};
use nacos_sdk::api::config::ConfigServiceBuilder;
use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, ConfigService};
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{
    NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder, ServiceInstance,
};
use nacos_sdk::api::props::ClientProps;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tracing::{error, info};

pub static CONFIG: OnceCell<AppConfig> = OnceCell::new();
#[allow(unused)]
#[derive(Clone)]
pub struct AppConfig {
    pub config: Arc<Config>,
    pub naming_service: Arc<dyn NamingService>,
    pub config_service: Arc<dyn ConfigService>,
}

#[allow(unused)]
impl AppConfig {
    pub async fn init(file_path: &str) -> Result<(), AppError> {
        let mut init_config = Config::builder()
            .add_source(File::with_name(file_path))
            .build()?;
        //nacos
        let client_props = ClientProps::new()
            .server_addr(init_config.get_string("nacos.server_addr")?)
            // .remote_grpc_port(9838)
            .app_name(init_config.get_string("app.name")?)
            // Attention! "public" is "", it is recommended to customize the namespace with clear meaning.
            .namespace(if init_config.get_string("nacos.namespace")?.eq("public") {
                "".to_string()
            } else {
                init_config.get_string("nacos.namespace")?
            })
            .auth_username(init_config.get_string("nacos.username")?)
            .auth_password(init_config.get_string("nacos.password")?);

        // ----------  Config  -------------
        let config_service = ConfigServiceBuilder::new(client_props.clone())
            .enable_auth_plugin_http()
            .build()
            .expect("Failed to build config service");
        let config_resp = config_service
            .get_config(
                init_config.get_string("nacos.config_id")?,
                constants::DEFAULT_GROUP.to_string(),
            )
            .await;
        match config_resp {
            Ok(config_resp) => {
                //重构 config
                info!("get the config");
                init_config = Config::builder()
                    .add_source(config::File::with_name(file_path))
                    .add_source(config::File::from_str(
                        config_resp.content(),
                        config::FileFormat::Yaml,
                    ))
                    .build()?;
            }
            Err(err) => error!("get the config {:?}", err),
        }
        let _listen = config_service
            .add_listener(
                init_config.get_string("nacos.config_id")?,
                constants::DEFAULT_GROUP.to_string(),
                std::sync::Arc::new(SimpleConfigChangeListener {}),
            )
            .await;
        match _listen {
            Ok(_) => info!("listening the config success"),
            Err(err) => error!("listen config error {:?}", err),
        }
        // ----------  Naming  -------------
        let naming_service = NamingServiceBuilder::new(client_props)
            .enable_auth_plugin_http()
            .build()
            .expect("Failed to build naming service");
        let listener = std::sync::Arc::new(SimpleInstanceChangeListener);
        let _subscribe_ret = naming_service
            .subscribe(
                init_config.get_string("app.name")?,
                Some(constants::DEFAULT_GROUP.to_string()),
                Vec::default(),
                listener,
            )
            .await;

        let service_instance = ServiceInstance {
            ip: init_config.get_string("app.ip")?,
            port: init_config.get_int("app.port")? as i32,
            ..Default::default()
        };
        let _register_instance_ret = naming_service
            .batch_register_instance(
                init_config.get_string("app.name")?,
                Some(constants::DEFAULT_GROUP.to_string()),
                vec![service_instance],
            )
            .await;
        info!("final config:{:?}", init_config);
        let app_config = AppConfig {
            config: Arc::from(init_config),
            naming_service: Arc::from(naming_service),
            config_service:Arc::from(config_service),
        };
        CONFIG.set(app_config);
        Ok(())
    }
}
struct SimpleConfigChangeListener;

impl ConfigChangeListener for SimpleConfigChangeListener {
    fn notify(&self, config_resp: ConfigResponse) {
        info!("listen the config={}", config_resp);
    }
}

pub struct SimpleInstanceChangeListener;

impl NamingEventListener for SimpleInstanceChangeListener {
    fn event(&self, event: std::sync::Arc<NamingChangeEvent>) {
        info!("subscriber notify: {:?}", event);
    }
}
