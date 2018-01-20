use rocket_contrib::Template;
use rocket::http::RawStr;
use rocket::response::Redirect;

extern crate redis;
use self::redis::Commands;

use std::env;

extern crate uuid;
use self::uuid::Uuid;

// TODO: handle errors
fn redis_client() -> redis::Connection {
    let url = env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(url.as_str()).unwrap();
    client.get_connection().unwrap()
}

#[derive(Serialize)]
struct IndexContext<'a> {
    name: &'a str,
    items: Vec<&'a str>,
}

#[get("/<name>")]
fn index(name: &RawStr) -> Template {
    let context = IndexContext {
        name: name,
        items: vec!["foo", "bar"],
    };
    Template::render("index", &context)
}

fn create_new_id() -> String {
    let id: String = Uuid::new_v4().to_string();

    let _: () = redis_client().set_ex(&id, 42, 600).unwrap();

    id
}

#[post("/")]
fn create_url() -> Redirect {
    let url = format!("/url/{id}", id = create_new_id());
    Redirect::to(&url)
}

#[derive(Serialize)]
struct UrlPage<'a> {
    id: &'a str,
}

#[get("/<id>")]
fn show_url(id: &RawStr) -> Template {
    let context = UrlPage { id };
    let res: Result<u8, redis::RedisError> = redis_client().exists(&id.to_string());

    let template_name = match res {
        Ok(n) => match n {
            1 => "url",
            _ => "expired",
        },
        Err(_) => "expired",
    };

    Template::render(template_name, &context)
}
