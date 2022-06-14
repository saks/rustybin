use rocket::data::{Data, ToByteUnit};
use rocket::request::Request;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Dump {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub url_params: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<String>,
    pub body_params: Option<HashMap<String, String>>,
    pub time: String,
    pub is_json: bool,
}

impl Dump {
    pub async fn add_data(&mut self, data: Data<'_>) {
        let mut body_data = Vec::new();

        // TODO: handle errors
        if data
            .open(512.kibibytes())
            .stream_to(&mut body_data)
            .await
            .is_ok()
        {
            self.body = Some(String::from_utf8_lossy(&body_data).to_string());
        }

        if self.is_json() {
            self.is_json = true;
        }
    }

    fn is_json(&self) -> bool {
        let context_type = self.headers.get("Content-Type");
        context_type == Some(&String::from("application/json"))
    }
}

fn time_str() -> String {
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .expect("to format time")
}

fn url_params(request: &Request) -> HashMap<String, String> {
    let mut url_params = HashMap::new();
    let origin = request.uri();

    let uri = format!("http://a.b/{origin}");

    if let Ok(parsed_uri) = Url::parse(&uri) {
        for (key, value) in parsed_uri.query_pairs() {
            url_params.insert(key.into(), value.into());
        }
    }

    url_params
}

fn headers(request: &Request) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    for header in request.headers().iter() {
        headers.insert(header.name().to_string(), header.value().to_string());
    }

    headers
}

fn cookies(request: &Request) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let cookies_obj = request.cookies();

    for c in cookies_obj.iter() {
        result.insert(c.name().to_string(), c.value().to_string());
    }

    result
}

impl<'a, 'r> From<&'a Request<'r>> for Dump {
    fn from(request: &'a Request<'r>) -> Self {
        // cut first 37 chars out
        let uri = request.uri().path().to_string()[37..].to_string();
        Self {
            method: request.method().to_string(),
            uri,
            headers: headers(&request),
            url_params: url_params(&request),
            cookies: cookies(&request),
            body: None,
            body_params: None,
            time: time_str(),
            is_json: false,
        }
    }
}
