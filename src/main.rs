#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Template;

mod app;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![app::index])
        .mount("/url", routes![app::show_url, app::create_url])
        .launch();
}
