extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

extern crate rustybin;
use rustybin::{app, apps};

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", app::app())
        .mount("/bins", apps::bins::app())
        .mount("/admin", apps::admin::app())
        .launch();
}
