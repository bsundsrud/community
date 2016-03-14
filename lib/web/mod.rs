use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use mount::Mount;
use router::Router;
use models::setup_connection_pool;
use iron::typemap::Key;
use models::PostgresPool;
use persistent::Read;

pub struct AppDb;
impl Key for AppDb { type Value = PostgresPool; }

#[derive(Debug, RustcEncodable)]
struct StatusResponse {
    status: String,
}

fn create_router() -> Router {
    let mut r = Router::new();
    r.get("/status", status_endpoint);
    r
}

fn create_middleware(r: Router) -> Chain {
    let mut mount = Mount::new();
    mount.mount("/", r);
    Chain::new(mount)
}

fn status_endpoint(_: &mut Request) -> IronResult<Response> {
    let status = StatusResponse { status: "Yep.".into() };
    let payload = json::encode(&status).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

pub fn start_server() {
    let pool = setup_connection_pool(2);
    let mut middleware = create_middleware(create_router());
    middleware.link(Read::<AppDb>::both(pool));
    Iron::new(middleware).http("localhost:3000").unwrap();
}
