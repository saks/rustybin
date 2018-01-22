extern crate failure;

use rocket::response::Redirect;
use rocket::http::RawStr;
use rocket::Route;
use std::path::PathBuf;
use self::failure::Error;

use rocket_contrib::Template;
extern crate serde_json;

use models::{Bin, Dump};
use render_with_layout::render_with_layout;

pub fn app() -> Vec<Route> {
    routes![index, show, create, capture_get, capture_post, delete]
}

#[get("/")]
fn index() -> Template {
    let page = match Bin::all() {
        Ok(bins) => IndexPage::success(bins),
        Err(err) => IndexPage::from_err(err),
    };
    render_with_layout("bins/index", page)
}

#[post("/")]
fn create() -> Redirect {
    match Bin::create() {
        Ok(bin) => Redirect::to(&format!("/{}", bin.id)),
        Err(err) => {
            println!("{}", err);
            Redirect::to("/")
        }
    }
}

#[get("/<id>/capture/<_path..>")]
fn capture_get(id: String, _path: PathBuf, dump: Dump) -> &'static str {
    match Bin::capture(id, dump) {
        Ok(_) => "OK",
        Err(_) => "expired", // TODO: respond with 404
    }
}

#[post("/<id>/capture/<_path..>", data = "<input>")]
fn capture_post(id: String, _path: PathBuf, mut dump: Dump, input: String) -> &'static str {
    dump.body = Some(input);

    match Bin::capture(id, dump) {
        Ok(_) => "OK",
        Err(_) => "expired", // TODO: respond with 404
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

#[delete("/<id>")]
fn delete(id: &RawStr) -> Redirect {
    let _ = Bin::delete(&id);
    Redirect::to("/")
}

#[derive(Serialize)]
struct ExpiredPage {
    msg: String,
}

#[derive(Serialize)]
struct IndexPage {
    bins: Vec<Bin>,
    err: Option<String>,
}

impl IndexPage {
    fn success(bins: Vec<Bin>) -> Self {
        Self {
            bins: bins,
            err: None,
        }
    }

    fn from_err(err: Error) -> Self {
        Self {
            bins: vec![],
            err: Some(format!("{}", err)),
        }
    }
}
