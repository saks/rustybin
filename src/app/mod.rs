extern crate failure;
extern crate serde;

use rocket_contrib::Template;

use render_with_layout::render_with_layout;

#[get("/")]
fn index() -> Template {
    render_with_layout("index", IndexContext {})
}

#[derive(Serialize)]
struct IndexContext;
