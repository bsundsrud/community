use dotenv::dotenv;
use std::env;
use postgres::{Connection, SslMode};


pub fn establish_connection() -> Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::connect(&*database_url, SslMode::None).unwrap()

}
