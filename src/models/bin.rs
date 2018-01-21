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
    #[fail(display = "Bin `{}' has already expired", id)] Expired { id: String },
}

#[derive(Serialize, Debug)]
pub struct Bin {
    pub id: String,
}

fn get_redis_client() -> Result<Connection, Error> {
    let url = env::var("REDIS_URL")?;
    let client = Client::open(url.as_str())?;
    let connection = client.get_connection()?;

    let timeout = Some(Duration::from_secs(5));

    connection.set_read_timeout(timeout)?;
    connection.set_write_timeout(timeout)?;

    Ok(connection)
}

impl Bin {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn create() -> Result<Self, Error> {
        let redis_client = get_redis_client()?;
        let id = Uuid::new_v4().to_string();

        // ignore return value
        let _: () = redis_client.set_ex(&id, 42, 600)?;

        Ok(Self::new(id))
    }

    pub fn find<'a>(id: &'a str) -> Result<Self, Error> {
        let redis_client = get_redis_client()?;
        let result = redis_client.exists(id)?;

        match result {
            1 => Ok(Self::new(id.to_string())),
            _ => Err(Errors::Expired { id: id.to_string() }.into()),
        }
    }

    pub fn all() -> Result<Vec<Self>, Error> {
        let redis_client = get_redis_client()?;
        let all_keys: Vec<String> = redis_client.keys("**")?;

        Ok(all_keys.into_iter().map(Self::new).collect())
    }
}
