use rocket::Route;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

pub fn app() -> Vec<Route> {
    routes![files]
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
