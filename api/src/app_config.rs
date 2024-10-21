use config::{Config, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;

pub static CONFIG: OnceCell<AppConfig> = OnceCell::new();

#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct IgnoreUrls {
    pub ignores: Vec<String>,
}

#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
    pub global: IgnoreUrls,
}

#[allow(unused)]
impl AppConfig {
    pub fn init(file_path: &str) {
        let settings = Config::builder()
            .add_source(File::with_name(file_path))
            .build()
            .unwrap();
        let config = settings.try_deserialize::<AppConfig>().unwrap();
        CONFIG.set(config);
    }
}
