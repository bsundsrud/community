use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use mount::Mount;
use router::Router;
use db::{PostgresPool, setup_connection_pool};
use iron::typemap::Key;
use persistent::Read;
use dotenv::dotenv;
use std::env;
use bodyparser;
#[macro_use]
mod macros;
mod orgs;

pub struct AppDb;
impl Key for AppDb { type Value = PostgresPool; }

#[derive(Debug, RustcEncodable)]
struct StatusResponse {
    status: String,
}

fn create_router() -> Router {
    let mut r = Router::new();
    r.get("/status", status_endpoint);
    r = orgs::register_endpoints(r);
    r
}

fn create_middleware(r: Router) -> Chain {
    let mut mount = Mount::new();
    mount.mount("/", r);
    let mut chain = Chain::new(mount);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(1024 * 1024 * 10));
    chain
}

fn status_endpoint(_: &mut Request) -> IronResult<Response> {
    let status = StatusResponse { status: "Yep.".into() };
    let payload = itry!(json::encode(&status));
    Ok(Response::with((status::Ok, payload)))
}

pub fn start_server() {
    dotenv().ok();
    let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Creating DB Pool...");
    let pool = setup_connection_pool(&*conn_str, 2).expect("Could not set up DB Pool.");
    let mut middleware = create_middleware(create_router());
    middleware.link(Read::<AppDb>::both(pool));
    info!("Starting server...");
    let server_host = "localhost:3000";
    match Iron::new(middleware).http(server_host) {
        Ok(_) => {
            info!("Server listening on {}", server_host);
        },
        Err(e) => {
            error!("Error: {}", e);
        }
    };
}
