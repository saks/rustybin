use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::http::RawStr;

extern crate failure;

mod model;
use self::model::Url;

#[get("/")]
fn index() -> Template {
    Template::render("index", IndexContext {})
}

#[post("/")]
fn create_url() -> Redirect {
    match Url::create() {
        Ok(id) => Redirect::to(&format!("/url/{}", id)),
        Err(_) => Redirect::to("/"),
    }
}

#[get("/<id>")]
fn show_url(id: &RawStr) -> Template {
    match Url::find(&id) {
        Ok(id) => Template::render("url", &(UrlPage { id })),
        Err(err) => Template::render(
            "expired",
            &(ExpiredPage {
                msg: err.to_string(),
            }),
        ),
    }
}

#[derive(Serialize)]
struct IndexContext;

#[derive(Serialize)]
struct UrlPage<'a> {
    id: &'a str,
}

#[derive(Serialize)]
struct ExpiredPage {
    msg: String,
}
