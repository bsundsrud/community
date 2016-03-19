use postgres::error::ConnectError;
use r2d2::{Pool, PooledConnection, Config, InitializationError};
use r2d2_postgres::{PostgresConnectionManager, SslMode};

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

#[derive(Debug)]
pub enum PoolError {
    Connect(ConnectError),
    Init(InitializationError)
}

impl From<ConnectError> for PoolError {
    fn from(e: ConnectError) -> PoolError {
        PoolError::Connect(e)
    }
}

impl From<InitializationError> for PoolError {
    fn from(e: InitializationError) -> PoolError {
        PoolError::Init(e)
    }
}

pub fn setup_connection_pool(connection_str: &str, pool_size: u32) -> Result<PostgresPool, PoolError> {
    let manager = try!(PostgresConnectionManager::new(connection_str, SslMode::None));
    let config = Config::builder().pool_size(pool_size).build();
    Ok(try!(Pool::new(config, manager)))
}
