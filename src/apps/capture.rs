use rocket::Route;
use rocket::http::Method::*;

use models::id::Id;
use models::bin::Bin;
use models::dump::Dump;

use rocket::{Data, Request};
use rocket::handler::Outcome;
use rocket::http::Status;

fn capture(request: &Request, data: Data) -> Outcome<'static> {
    let id = Id::from(request);
    if id.is_valid() {
        if id.check_fresh().is_ok() {
            let mut dump = Dump::from(request);
            dump.add_data(&data);

            match Bin::capture(&id, &dump) {
                Ok(_) => Outcome::from(request, "OK"),
                Err(_) => Outcome::failure(Status::BadRequest),
            }
        } else {
            Outcome::failure(Status::BadRequest)
        }
    } else {
        Outcome::forward(data)
    }
}

pub fn app() -> Vec<Route> {
    [Get, Post, Put, Patch, Head, Delete]
        .into_iter()
        .map(|method| Route::ranked(10, *method, "/<path..>", capture))
        .collect()
}
