use std::env;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{debug, log};
use crate::db::REPOSITORY;

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
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug); // Setting default PostgreSQL schema

        let db = Database::connect(opt).await.unwrap();
        Repo { sea_orm: db }
    }
    #[tracing::instrument]
    pub async fn create() {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let repo = Repo::new(&database_url);

        REPOSITORY.set(repo.await).expect("db connection must set");

        debug!("db connection created");
    }
}
