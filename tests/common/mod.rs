extern crate rocket;
extern crate redis;
extern crate rustybin;

use self::rocket::local::Client;
use rustybin::redis::get_redis_client;
use rustybin::server;

pub fn reset_db() {
    let redis_connection = get_redis_client().unwrap();
    let _: () = redis::cmd("FLUSHALL").query(&redis_connection).unwrap();
}

pub fn client() -> Client {
    Client::new(server()).unwrap()
}
