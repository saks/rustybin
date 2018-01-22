extern crate redis;
extern crate rustybin;

use rustybin::redis::get_redis_client;

pub fn reset_db() {
    let redis_connection = get_redis_client().unwrap();
    let _: () = redis::cmd("FLUSHALL").query(&redis_connection).unwrap();
}
