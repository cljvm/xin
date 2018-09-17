use db::DbExecutor;
use config::Config;
use actix::prelude::*;


#[derive(Clone)]
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

impl AppState {
    pub fn new(config: &Config) -> AppState {
        let db = DbExecutor::new(&config.db);

        let addr = SyncArbiter::start(3, move || {
            db.clone()
        });

        AppState {db: addr}
    }
}