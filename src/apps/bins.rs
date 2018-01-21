use rocket::response::Redirect;
use rocket::http::RawStr;
use rocket::Route;

use rocket_contrib::Template;

use model::Url;
use render_with_layout::render_with_layout;

pub fn app() -> Vec<Route> {
    routes![show, create]
}

#[post("/")]
fn create() -> Redirect {
    match Url::create() {
        Ok(id) => Redirect::to(&format!("/bins/{}", id)),
        Err(err) => {
            println!("{}", err);
            Redirect::to("/")
        }
    }
}

#[get("/<id>")]
fn show(id: &RawStr) -> Template {
    let context = ShowPage { id };
    println!("-------------> {:?} <---------------", context);
    match Url::find(&id) {
        Ok(id) => render_with_layout("bins/show", &context),
        Err(err) => render_with_layout(
            "expired",
            &(ExpiredPage {
                msg: err.to_string(),
            }),
        ),
    }
}

#[derive(Serialize, Debug)]
struct ShowPage<'a> {
    id: &'a str,
}

#[derive(Serialize)]
struct ExpiredPage {
    msg: String,
}
