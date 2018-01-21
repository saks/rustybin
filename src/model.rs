extern crate failure;
extern crate redis;
extern crate uuid;

use std::time::Duration;
use self::redis::{Client, Commands, Connection};
use self::failure::Error;
use self::uuid::Uuid;
use std::env;

#[derive(Debug, Fail, Serialize)]
pub enum Errors {
    #[fail(display = "Url `{}' has already expired", id)] Expired { id: String },
}

pub struct Url;

fn get_redis_client() -> Result<Connection, Error> {
    let url = env::var("REDIS_URL")?;
    let client = Client::open(url.as_str())?;
    let connection = client.get_connection()?;

    let timeout = Some(Duration::from_secs(5));

    connection.set_read_timeout(timeout)?;
    connection.set_write_timeout(timeout)?;

    Ok(connection)
}

impl Url {
    pub fn create() -> Result<String, Error> {
        let redis_client = get_redis_client()?;
        let id = Uuid::new_v4().to_string();

        // ignore return value
        let _: () = redis_client.set_ex(&id, 42, 600)?;

        Ok(id)
    }

    pub fn find<'a>(id: &'a str) -> Result<&'a str, Error> {
        let redis_client = get_redis_client()?;
        let result = redis_client.exists(id)?;

        match result {
            1 => Ok(id),
            _ => Err(Errors::Expired { id: id.to_string() }.into()),
        }
    }

    pub fn all() -> Result<Vec<String>, Error> {
        let redis_client = get_redis_client()?;
        Ok(redis_client.keys("**")?)
    }
}
