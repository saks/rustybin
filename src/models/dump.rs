extern crate time;
extern crate url;

use rocket::Data;
use rocket::request::Request;
use std::collections::HashMap;
use self::url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dump {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub url_params: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<String>,
    pub body_params: Option<HashMap<String, String>>,
    pub time: String,
}

impl Dump {
    pub fn add_data(&mut self, data: &Data) {
        self.body = Some(String::from_utf8_lossy(data.peek()).to_string());
    }
}

fn time_str() -> String {
    let now = time::get_time();
    let utc = time::at_utc(now);

    utc.rfc3339().to_string()
}

fn url_params<'a, 'r>(request: &'a Request<'r>) -> HashMap<String, String> {
    let mut url_params = HashMap::new();

    let uri = "http://a.b/".to_string() + request.uri().as_str();

    if let Ok(parsed_uri) = Url::parse(&uri) {
        for (key, value) in parsed_uri.query_pairs() {
            url_params.insert(key.into(), value.into());
        }
    }

    url_params
}

fn headers<'a, 'r>(request: &'a Request<'r>) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    for header in request.headers().iter() {
        headers.insert(header.name().to_string(), header.value().to_string());
    }

    headers
}

fn cookies<'a, 'r>(request: &'a Request<'r>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let cookies_obj = request.cookies();

    for c in cookies_obj.iter() {
        result.insert(c.name().to_string(), c.value().to_string());
    }

    result
}

impl<'a, 'r> From<&'a Request<'r>> for Dump {
    fn from(request: &'a Request<'r>) -> Self {
        Self {
            method: request.method().to_string(),
            uri: String::from(&request.uri().path()[37..]), // cut first 37 chars out
            headers: headers(&request),
            url_params: url_params(&request),
            cookies: cookies(&request),
            body: None,
            body_params: None,
            time: time_str(),
        }
    }
}
