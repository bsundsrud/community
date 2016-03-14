use dotenv::dotenv;
use std::env;
use postgres::{Connection, SslMode};
use r2d2::{Pool, PooledConnection, Config};
use r2d2_postgres::PostgresConnectionManager;

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

pub fn setup_connection_pool(pool_size: u32) -> PostgresPool {
    dotenv().ok();
    let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new(&*conn_str, ::r2d2_postgres::SslMode::None)
                      .unwrap();
    let config = Config::builder().pool_size(pool_size).build();
    Pool::new(config, manager).unwrap()
}

pub fn establish_connection() -> Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::connect(&*database_url, SslMode::None).unwrap()

}
