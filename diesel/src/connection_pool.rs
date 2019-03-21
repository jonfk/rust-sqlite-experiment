use diesel::connection::Connection;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use failure::Error;
use r2d2::{Pool, PooledConnection};
use std::sync::{Arc, Mutex, MutexGuard};

pub struct SqliteConnectionPool {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl SqliteConnectionPool {
    pub fn new_from_path(db_path: &str) -> Result<SqliteConnectionPool, Error> {
        let conn: SqliteConnection = Connection::establish(db_path)?;
        Ok(SqliteConnectionPool {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn get(&self) -> Result<Arc<Mutex<SqliteConnection>>, Error> {
        Ok(self.conn)
    }
}
