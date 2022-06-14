use rocket::{get, routes, Route};
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

pub fn app() -> Vec<Route> {
    routes![files]
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
