use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::collections::HashMap;
extern crate time;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dump {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub time: String,
}

fn time_str() -> String {
    let now = time::get_time();
    let utc = time::at_utc(now);
    let tm = utc.rfc3339();
    format!("{}", tm)
}

impl<'a, 'r> FromRequest<'a, 'r> for Dump {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Dump, ()> {
        let uri = request.uri().as_str();
        let mut headers = HashMap::new();

        for header in request.headers().iter() {
            headers
                .insert(header.name().to_string(), header.value().to_string());
        }

        let dump = Dump {
            method: request.method().to_string(),
            uri: String::from(&uri[37..]), // cut first 37 chars out
            headers: headers,
            body: None,
            time: time_str(),
        };
        return Outcome::Success(dump);
    }
}

impl<'a, 'r> From<&'a Request<'r>> for Dump {
    fn from(request: &'a Request<'r>) -> Self {
        let uri = request.uri().as_str();
        let mut headers = HashMap::new();

        for header in request.headers().iter() {
            headers
                .insert(header.name().to_string(), header.value().to_string());
        }

        Self {
            method: request.method().to_string(),
            uri: String::from(&uri[37..]), // cut first 37 chars out
            headers: headers,
            body: None,
            time: time_str(),
        }
    }
}
