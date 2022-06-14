use std::fmt;

use failure::{Error, Fail};
use rocket::request::Request;
use serde::Serialize;

use crate::redis::{get_redis_client, Commands};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Id {
    uuid: Option<Uuid>,
}

impl<'a, 'r> From<&'a Request<'r>> for Id {
    fn from(request: &'a Request<'r>) -> Self {
        let uri_segments = request.uri().path().segments();

        if let Some(id_str) = uri_segments.take(1).nth(0) {
            if let Ok(uuid) = Uuid::parse_str(id_str) {
                return Id::new(uuid);
            }
        }
        Self::default()
    }
}

#[derive(Debug, Fail, Serialize)]
pub enum Errors {
    #[fail(display = "Id has already expired")]
    Expired,
    #[fail(display = "Id is empty")]
    Empty,
}

impl Id {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid: Some(uuid) }
    }

    pub fn is_valid(&self) -> bool {
        self.uuid.is_some()
    }

    pub fn check_fresh(&self) -> Result<(), Error> {
        if self.uuid.is_none() {
            return Err(Errors::Empty {}.into());
        }

        let mut redis_client = get_redis_client()?;

        let id = format!("{}", self.uuid.unwrap());

        let exist_res: u8 = redis_client.exists(id)?;
        if 0 == exist_res {
            return Err(Errors::Expired {}.into());
        }

        Ok(())
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_valid() {
            write!(f, "{}", self.uuid.unwrap())
        } else {
            write!(f, "{:?}", self.uuid)
        }
    }
}
