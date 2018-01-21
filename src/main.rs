#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate failure;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

pub mod render_with_layout;
pub mod models;
mod app;
mod apps;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![app::index])
        .mount("/bins", apps::bins::app())
        .mount("/admin", apps::admin::app())
        .launch();
}
