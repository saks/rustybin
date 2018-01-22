extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

extern crate rustybin;
use rustybin::apps;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", apps::bins::app())
        .launch();
}
