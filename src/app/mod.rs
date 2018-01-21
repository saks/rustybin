extern crate failure;
extern crate serde;

use rocket_contrib::Template;
use rocket::Route;

use render_with_layout::render_with_layout;

pub fn app() -> Vec<Route> {
    routes![index]
}

#[get("/")]
fn index() -> Template {
    render_with_layout("index", IndexContext {})
}

#[derive(Serialize)]
struct IndexContext;
