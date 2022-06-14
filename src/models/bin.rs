use failure::{Error, Fail};
use serde::Serialize;
use uuid::Uuid;

use crate::models::dump::Dump;
use crate::models::id::Id;
use crate::redis::{get_redis_client, transaction, Commands};

const RECORD_TTL: u16 = 6000; // seconds

#[derive(Debug, Fail, Serialize)]
pub enum Errors {
    #[fail(display = "Bin `{}' has already expired", id)]
    Expired { id: String },
}

#[derive(Serialize, Debug)]
pub struct Bin {
    pub id: String,
    pub dumps: Vec<Dump>,
}

impl Bin {
    pub fn new(id: String) -> Self {
        Self { id, dumps: vec![] }
    }

    pub fn create() -> Result<Self, Error> {
        let mut redis_client = get_redis_client()?;
        let key = Uuid::new_v4().to_string();

        let _: () = transaction(&mut redis_client, &[&key], |conn, pipe| {
            pipe.cmd("DEL")
                .arg(&key)
                .cmd("LPUSH")
                .arg(&key)
                .arg("")
                .cmd("EXPIRE")
                .arg(&key)
                .arg(RECORD_TTL)
                .query(conn)
        })?;

        Ok(Self::new(key))
    }

    pub fn delete(id: &str) -> Result<(), Error> {
        let mut redis_client = get_redis_client()?;
        let _: () = redis_client.del(id)?;
        Ok(())
    }

    pub fn find(id: &str) -> Result<Self, Error> {
        let mut redis_client = get_redis_client()?;

        let exist_res: u8 = redis_client.exists(id)?;
        if 0 == exist_res {
            return Err(Errors::Expired { id: id.to_string() }.into());
        }

        let strings: Vec<String> = redis_client.lrange(id, 0, 10)?;
        let mut dumps = vec![];

        for data in strings {
            if !data.is_empty() {
                dumps.push(serde_json::from_str(&data)?);
            }
        }

        let mut res = Self::new(id.to_string());
        res.dumps = dumps;

        Ok(res)
    }

    pub fn all() -> Result<Vec<Self>, Error> {
        let mut redis_client = get_redis_client()?;
        let all_keys: Vec<String> = redis_client.keys("**")?;

        Ok(all_keys.into_iter().map(Self::new).collect())
    }

    pub fn capture(id: &Id, dump: &Dump) -> Result<(), Error> {
        let id = id.to_string();
        let mut redis_client = get_redis_client()?;

        let json = serde_json::to_string(&dump)?;

        let _: () = transaction(&mut redis_client, &[&id], |conn, pipe| {
            pipe.cmd("LPUSH")
                .arg(&id)
                .arg(&json)
                .cmd("EXPIRE")
                .arg(&id)
                .arg(RECORD_TTL)
                .query(conn)
        })?;

        Ok(())
    }
}
