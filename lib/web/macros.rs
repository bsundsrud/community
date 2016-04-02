
macro_rules! get_pg_connection {
    ($req:expr) => (match $req.get::<::persistent::Read<::web::AppDb>>() {
        Ok(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                println!("Couldn't get a connection to pg!");
                return Ok(Response::with((::iron::status::InternalServerError)));
            }
        },
        Err(_) => {
            println!("Couldn't get the pg pool from the request!");
            return Ok(Response::with((::iron::status::InternalServerError)));
        }
    })
}
