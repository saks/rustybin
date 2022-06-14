use failure::Error;
use rocket::response::Redirect;
use rocket::Route;
use rocket::{delete, get, post, routes};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::models::Bin;
use crate::render_with_layout::render_with_layout;

pub fn app() -> Vec<Route> {
    routes![index, show, create, delete]
}

#[get("/", rank = 2)]
async fn index() -> Template {
    let page = match Bin::all() {
        Ok(bins) => IndexPage::success(bins),
        Err(err) => IndexPage::from_err(&err),
    };
    render_with_layout("bins/index", page)
}

#[post("/", rank = 2)]
async fn create() -> Redirect {
    match Bin::create() {
        Ok(_bin) => Redirect::to("/"),
        Err(err) => {
            println!("{}", err);
            Redirect::to("/")
        }
    }
}

#[get("/__rustybin/<id>", rank = 2)]
async fn show(id: &str) -> Template {
    match Bin::find(id) {
        Ok(bin) => render_with_layout("bins/show", bin),
        Err(err) => render_with_layout(
            "expired",
            &(ExpiredPage {
                msg: err.to_string(),
            }),
        ),
    }
}

#[delete("/__rustybin/<id>", rank = 2)]
async fn delete(id: &str) -> Redirect {
    let _ = Bin::delete(id);
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
        Self { bins, err: None }
    }

    fn from_err(err: &Error) -> Self {
        Self {
            bins: vec![],
            err: Some(format!("{}", err)),
        }
    }
}
