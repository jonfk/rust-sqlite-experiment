use failure::Error;
use r2d2::{self, Pool, PooledConnection};
use rusqlite::{self, Connection};
use std::path::PathBuf;
use std::result;

#[derive(Clone)]
pub struct SqliteConnectionPool(Pool<SqliteConnectionManager>);

pub struct SqliteConnectionManager {
    path: PathBuf,
    flags: Option<rusqlite::OpenFlags>,
}

impl r2d2::ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> result::Result<Connection, rusqlite::Error> {
        match self {
            &SqliteConnectionManager {
                path: ref path,
                flags: None,
            } => Connection::open(path),
            &SqliteConnectionManager {
                path: ref path,
                flags: Some(flags),
            } => Connection::open_with_flags(path, flags),
        }
        .map_err(Into::into)
    }

    fn is_valid(&self, conn: &mut Connection) -> result::Result<(), rusqlite::Error> {
        conn.execute_batch("").map_err(Into::into)
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}

impl SqliteConnectionManager {
    pub fn new_with_flags<T>(path: T, flags: rusqlite::OpenFlags) -> SqliteConnectionManager
    where
        T: Into<PathBuf>,
    {
        SqliteConnectionManager {
            path: path.into(),
            flags: Some(flags),
        }
    }

    pub fn new<T>(path: T) -> SqliteConnectionManager
    where
        T: Into<PathBuf>,
    {
        SqliteConnectionManager {
            path: path.into(),
            flags: None,
        }
    }
}

impl SqliteConnectionPool {
    pub fn new_from_path(path: &str) -> Result<SqliteConnectionPool, Error> {
        use rusqlite::OpenFlags;
        let manager = SqliteConnectionManager::new_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX
                | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
        );
        let pool = r2d2::Pool::new(manager)?;
        let mut conn = pool.get()?;
        Ok(SqliteConnectionPool(pool))
    }

    pub fn get(&self) -> Result<PooledConnection<SqliteConnectionManager>, Error> {
        Ok(self.0.get()?)
    }
}
