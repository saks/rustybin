#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket_contrib::Template;
// use handlebars::{Helper, Handlebars, RenderContext, RenderError, JsonRender};

// #[derive(Serialize)]
// struct TemplateContext {
//     name: String,
//     items: Vec<String>
// }
//

use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome::*;
use rocket::http::RawStr;


use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
struct MyReq {
    s: String,
}

impl fmt::Display for MyReq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for MyReq {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let s = format!("{:?}", request.headers());
        Success(MyReq { s })
    }
}

#[derive(Serialize)]
struct IndexContext<'a> {
    name: &'a str,
    items: Vec<&'a str>,
}

#[get("/<name>")]
fn index(name: &RawStr) -> Template {
    // format!("{}", req)
    // let mut context: HashMap<&str, &str> = HashMap::new();
    // context.insert("name", "Alex");
    let context = IndexContext { name: name, items: vec!["foo", "bar"] };
    Template::render("index", &context)
}

mod app;

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![app::index]).launch();
}
