use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use failure::Error;
use r2d2::{Pool, PooledConnection};

#[derive(Clone)]
pub struct SqliteConnectionPool(Pool<ConnectionManager<SqliteConnection>>);

impl SqliteConnectionPool {
    pub fn new_from_path(db_path: &str) -> Result<SqliteConnectionPool, Error> {
        let manager = ConnectionManager::<SqliteConnection>::new(db_path);
        let pool = r2d2::Pool::builder().build(manager)?;
        Ok(SqliteConnectionPool(pool))
    }

    pub fn get(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, Error> {
        let conn = self.0.get()?;
        Ok(conn)
    }
}
