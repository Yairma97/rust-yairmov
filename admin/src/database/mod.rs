
pub mod user;

use crate::config::CONFIG;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;
use tracing::{debug, log};

pub static REPOSITORY: OnceCell<Repo> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct Repo {
    pub(crate) sea_orm: DatabaseConnection,
}

impl Repo {
    async fn new(database_url: &str) -> Self {
        Self::sea_orm(database_url).await
    }

    async fn sea_orm(database_url: &str) -> Self {
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info); // Setting default PostgreSQL schema

        let db = Database::connect(opt).await.unwrap();
        Repo { sea_orm: db }
    }
    #[tracing::instrument]
    pub async fn create() {
        let app_config = CONFIG.get().unwrap();
        let database_url = &app_config.app.database_url;;

        let repo = Repo::new(database_url);

        REPOSITORY.set(repo.await).expect("db connection must set");

        debug!("db connection created");
    }
}
