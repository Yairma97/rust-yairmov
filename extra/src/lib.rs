use tracing_appender::non_blocking::WorkerGuard;

pub mod logger;
#[cfg(feature = "lib_redis")]
pub mod redis;
pub mod jwt;


pub async fn init() -> (WorkerGuard, WorkerGuard) {
    let (_guard_file, _guard_stderr) = logger::log_create();

    #[cfg(feature = "lib_redis")]
    redis::connection::RedisConnection::create().await;

    (_guard_file, _guard_stderr)
}
