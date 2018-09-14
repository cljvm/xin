use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };
use error::Error;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn conn(&mut self) -> Result<Conn, Error> {
        Ok(self.0.get().map_err(Error::database_connection_get_error)?)
    }
}