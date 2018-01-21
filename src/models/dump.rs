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
    pub time: i64,
}

impl<'a, 'r> FromRequest<'a, 'r> for Dump {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Dump, ()> {
        let uri = request.uri().as_str();
        let mut headers = HashMap::new();

        for header in request.headers().iter() {
            headers.insert(header.name().to_string(), header.value().to_string());
        }

        let dump = Dump {
            method: request.method().to_string(),
            uri: String::from(&uri[50..]), // cut first 50 chars out
            headers: headers,
            body: None,
            time: time::get_time().sec,
        };
        return Outcome::Success(dump);
    }
}
