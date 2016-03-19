#[macro_use]
extern crate log;
extern crate env_logger;
extern crate community;
use community::web;

fn main() {
    env_logger::init().unwrap();
    web::start_server();
}
