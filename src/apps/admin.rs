extern crate failure;
use self::failure::Error;

use rocket::Route;
use model::Bin;

use rocket_contrib::Template;

use render_with_layout::render_with_layout;

pub fn app() -> Vec<Route> {
    routes![index]
}

#[get("/")]
fn index() -> Template {
    let page = match Bin::all() {
        Ok(bins) => IndexPage::success(bins),
        Err(err) => IndexPage::from_err(err),
    };
    render_with_layout("admin/index", page)
}

#[derive(Serialize)]
struct IndexPage {
    bins: Option<Vec<String>>,
    err: Option<String>,
}

impl IndexPage {
    fn success(bins: Vec<String>) -> Self {
        Self {
            bins: Some(bins),
            err: None,
        }
    }

    fn from_err(err: Error) -> Self {
        Self {
            bins: None,
            err: Some(format!("{}", err)),
        }
    }
}
