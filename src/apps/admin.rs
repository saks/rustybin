extern crate failure;
use self::failure::Error;

use rocket::Route;
use models::Bin;

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
