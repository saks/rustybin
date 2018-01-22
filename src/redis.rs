extern crate failure;
extern crate redis;

use std::time::Duration;
use self::redis::{Client, Connection};
use self::failure::Error;
use std::env;

pub use self::redis::{transaction, Commands};

pub fn get_redis_client() -> Result<Connection, Error> {
    let url = env::var("REDIS_URL")?;
    let client = Client::open(url.as_str())?;
    let connection = client.get_connection()?;

    let timeout = Some(Duration::from_secs(5));

    connection.set_read_timeout(timeout)?;
    connection.set_write_timeout(timeout)?;

    Ok(connection)
}
