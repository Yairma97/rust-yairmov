use once_cell::sync::OnceCell;

use self::connection::RedisConnection;

pub mod connection;

static REDIS: OnceCell<RedisConnection> = OnceCell::new();
