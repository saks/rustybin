use rocket::launch;

use crate::apps;
use rocket_dyn_templates::Template;

#[launch]
pub fn server() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", apps::static_files::app())
        .mount("/", apps::bins::app())
    // .mount("/", apps::capture::app())
}
