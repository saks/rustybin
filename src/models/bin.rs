extern crate failure;
extern crate redis;
extern crate uuid;

use std::time::Duration;
use self::redis::{transaction, Client, Commands, Connection};
use self::failure::Error;
use self::uuid::Uuid;
use std::env;

extern crate serde_json;

use models::dump::Dump;

const RECORD_TTL: u16 = 6000; // seconds

#[derive(Debug, Fail, Serialize)]
pub enum Errors {
    #[fail(display = "Bin `{}' has already expired", id)] Expired { id: String },
}

#[derive(Serialize, Debug)]
pub struct Bin {
    pub id: String,
    pub dumps: Vec<Dump>,
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
        Self { id, dumps: vec![] }
    }

    pub fn create() -> Result<Self, Error> {
        let redis_client = get_redis_client()?;
        let key = Uuid::new_v4().to_string();

        let _: () = transaction(&redis_client, &[&key], |pipe| {
            pipe.cmd("DEL")
                .arg(&key)
                .cmd("LPUSH")
                .arg(&key)
                .arg("")
                .cmd("EXPIRE")
                .arg(&key)
                .arg(RECORD_TTL)
                .query(&redis_client)
        })?;

        Ok(Self::new(key))
    }

    pub fn delete(id: &str) -> Result<(), Error> {
        let redis_client = get_redis_client()?;
        let _: () = redis_client.del(id)?;
        Ok(())
    }

    pub fn find<'a>(id: &'a str) -> Result<Self, Error> {
        let redis_client = get_redis_client()?;

        let exist_res: u8 = redis_client.exists(id)?;
        if 0 == exist_res {
            return Err(Errors::Expired { id: id.to_string() }.into());
        }

        let strings: Vec<String> = redis_client.lrange(id, 0, 10)?;
        let mut dumps = vec![];

        for data in strings.into_iter() {
            println!("json: `{}'", data);
            if !data.is_empty() {
                dumps.push(serde_json::from_str(&data)?);
            }
        }

        let mut res = Self::new(id.to_string());
        res.dumps = dumps;

        println!("{:?}", &res);

        Ok(res)
    }

    pub fn all() -> Result<Vec<Self>, Error> {
        let redis_client = get_redis_client()?;
        let all_keys: Vec<String> = redis_client.keys("**")?;

        Ok(all_keys.into_iter().map(Self::new).collect())
    }

    pub fn capture(id: String, dump: Dump) -> Result<(), Error> {
        let redis_client = get_redis_client()?;

        let result: u8 = redis_client.exists(&id)?;

        if 0 == result {
            return Err(Errors::Expired { id: id.to_string() }.into());
        }

        let json = serde_json::to_string(&dump)?;

        let _: () = transaction(&redis_client, &[&id], |pipe| {
            pipe.cmd("LPUSH")
                .arg(&id)
                .arg(&json)
                .cmd("EXPIRE")
                .arg(&id)
                .arg(RECORD_TTL)
                .query(&redis_client)
        })?;

        Ok(())
    }
}
