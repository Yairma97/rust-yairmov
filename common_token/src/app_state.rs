use std::sync::Arc;

use dashmap::DashMap;

#[derive(Debug)]
pub struct Context {
    pub context: DashMap<String, String>,
}

pub type AppState = Arc<Context>;



