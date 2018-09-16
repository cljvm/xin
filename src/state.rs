use db::DbExecutor;
use config::Config;
use actix::prelude::*;


pub(crate) struct AppState {
    db: Addr<DbExecutor>,
}

impl AppState {
    pub fn new(config: Config) {
        let db = DbExecutor.new(config.dbConfig);

        let addr = SyncArbiter::start(3, move || {
            let cloned = db.cloned();
            DbExecutor(cloned)
        });

        AppState {db: addr}
    }
}