extern crate time;
extern crate url;

use rocket::request::Request;
use std::collections::HashMap;
use self::url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dump {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub url_params: HashMap<String, String>,
    pub body: Option<String>,
    pub time: String,
}

fn time_str() -> String {
    let now = time::get_time();
    let utc = time::at_utc(now);

    utc.rfc3339().to_string()
}

impl<'a, 'r> From<&'a Request<'r>> for Dump {
    fn from(request: &'a Request<'r>) -> Self {
        let mut headers = HashMap::new();

        for header in request.headers().iter() {
            headers
                .insert(header.name().to_string(), header.value().to_string());
        }

        let mut url_params = HashMap::new();
        let uri = "http://abc.com/".to_string() + request.uri().as_str();
        if let Ok(parsed_uri) = Url::parse(&uri) {
            for (key, value) in parsed_uri.query_pairs() {
                url_params.insert(key.into(), value.into());
            }
        }

        Self {
            method: request.method().to_string(),
            uri: String::from(&request.uri().path()[37..]), // cut first 37 chars out
            headers: headers,
            url_params: url_params,
            body: None,
            time: time_str(),
        }
    }
}
