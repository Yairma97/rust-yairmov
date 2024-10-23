pub async fn start() {
    dotenv::dotenv().ok();

    // add return value here, log can output to console and file, don't change.
    // issue: https://github.com/tokio-rs/tracing/issues/971
    let (_guard_file, _guard_stderr) = extra::init().await;

    database::db::Repo::create().await;

    api::start().await;
}
