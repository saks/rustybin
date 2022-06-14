extern crate redis;
extern crate rocket;
extern crate rustybin;

use rocket::local::blocking::Client;
use rustybin::redis::get_redis_client;
use rustybin::server;

pub fn reset_db() {
    let mut redis_connection = get_redis_client().expect("connect to redis");
    let _: () = redis::cmd("FLUSHALL")
        .query(&mut redis_connection)
        .expect("flush db");
}

#[allow(dead_code)]
pub fn client() -> Client {
    Client::untracked(server()).unwrap()
}
