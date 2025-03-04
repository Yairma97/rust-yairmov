use crate::config::CONFIG;
use once_cell::sync::OnceCell;
use redis::aio::ConnectionManager;
use std::{env, fmt};
use tracing::debug;

static REDIS: OnceCell<RedisConnection> = OnceCell::new();

pub struct RedisConnection {
    pub(crate) connect_manager: ConnectionManager,
}

impl RedisConnection {
    async fn new(redis_url: &str) -> Self {
        let cm = redis::Client::open(redis_url)
            .unwrap()
            .get_tokio_connection_manager()
            .await
            .unwrap();
        Self {
            connect_manager: cm,
        }
    }

    pub async fn create() {
        let redis_url = &CONFIG.get().unwrap().app.redis_url;
        debug!(target: "redis connection", redis_url = ?redis_url);
        let redis_connection = RedisConnection::new(redis_url);
        REDIS
            .set(redis_connection.await)
            .expect("redis conn must set");
        debug!("redis connection created");
    }
}

impl fmt::Debug for RedisConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedisConnection").finish()
    }
}
