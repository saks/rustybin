extern crate failure;
extern crate redis;

use model::Url;

#[get("/")]
fn index() -> String {
    match Url::all() {
        Ok(urls) => format!("{:?}", urls),
        Err(err) => err.to_string(),
    }
}
