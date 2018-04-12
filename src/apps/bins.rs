extern crate failure;

use self::failure::Error;
use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket::Route;

use rocket_contrib::Template;
extern crate serde_json;

use models::Bin;
use render_with_layout::render_with_layout;

pub fn app() -> Vec<Route> {
    routes![index, show, create, delete]
}

#[get("/", rank = 2)]
fn index() -> Template {
    let page = match Bin::all() {
        Ok(bins) => IndexPage::success(bins),
        Err(err) => IndexPage::from_err(&err),
    };
    render_with_layout("bins/index", page)
}

#[post("/", rank = 2)]
fn create() -> Redirect {
    match Bin::create() {
        Ok(_bin) => Redirect::to("/"),
        Err(err) => {
            println!("{}", err);
            Redirect::to("/")
        }
    }
}

#[get("/__rustybin/<id>", rank = 2)]
fn show(id: &RawStr) -> Template {
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
fn delete(id: &RawStr) -> Redirect {
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
        Self {
            bins: bins,
            err: None,
        }
    }

    fn from_err(err: &Error) -> Self {
        Self {
            bins: vec![],
            err: Some(format!("{}", err)),
        }
    }
}
