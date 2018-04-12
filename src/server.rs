extern crate rocket;
extern crate rocket_contrib;

use apps;
use rocket_contrib::Template;

pub fn server() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", apps::static_files::app())
        .mount("/", apps::capture::app())
        .mount("/", apps::bins::app())
}
