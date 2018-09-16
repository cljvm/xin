use std::thread;
use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use error::Error;
use config::DbConfig;
use actix::prelude::*;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(config: DbConfig) -> Result<DbExecutor> {
        let database_url = config.to_string();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .min_idle(Some(3))
            .build(manager)
            .expect("Failed to create pool.");

        for _ in 0..10i32 {
            let pool = pool.clone();
            thread::spawn(move || {
                let connection = pool.get();
                assert!(connection.is_ok());
            });
        }

        DbExecutor(pool)
    }

    pub fn conn(&mut self) -> Result<Conn, Error> {
        Ok(self.0.get().map_err(Error::database_connection_get_error)?)
    }
}
