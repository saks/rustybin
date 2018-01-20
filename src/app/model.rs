extern crate failure;
extern crate redis;
extern crate uuid;

use self::redis::{Client, Commands, Connection};
use self::failure::Error;
use self::uuid::Uuid;
use std::env;

pub struct Url;

fn get_redis_client() -> Result<Connection, Error> {
    let url = env::var("REDIS_URL")?;
    let client = Client::open(url.as_str())?;
    Ok(client.get_connection()?)
}

#[derive(Debug, Fail, Serialize)]
pub enum UrlError {
    #[fail(display = "expired url {}", id)] Expired { id: String },
}

impl Url {
    pub fn create() -> Result<String, Error> {
        let redis_client = get_redis_client()?;
        let id: String = Uuid::new_v4().to_string();

        let _: () = redis_client.set_ex(&id, 42, 600)?;

        Ok(id)
    }

    pub fn find(id: String) -> Result<String, Error> {
        let redis_client = get_redis_client()?;
        let res: u8 = redis_client.exists(&id.to_string())?;

        let error1 = UrlError::Expired { id: id.to_owned() };

        let error2 = error1.into();

        match res {
            1 => Ok(id),
            _ => Err(error2),
        }
    }
}
