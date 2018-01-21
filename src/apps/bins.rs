use rocket::response::Redirect;
use rocket::http::RawStr;
use rocket::Route;

use rocket_contrib::Template;

use models::Bin;
use render_with_layout::render_with_layout;

pub fn app() -> Vec<Route> {
    routes![show, create]
}

#[post("/")]
fn create() -> Redirect {
    match Bin::create() {
        Ok(bin) => Redirect::to(&format!("/bins/{}", bin.id)),
        Err(err) => {
            println!("{}", err);
            Redirect::to("/")
        }
    }
}

#[get("/<id>")]
fn show(id: &RawStr) -> Template {
    match Bin::find(&id) {
        Ok(bin) => render_with_layout("bins/show", bin),
        Err(err) => render_with_layout(
            "expired",
            &(ExpiredPage {
                msg: err.to_string(),
            }),
        ),
    }
}

#[derive(Serialize)]
struct ExpiredPage {
    msg: String,
}
