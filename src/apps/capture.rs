use crate::models::bin::Bin;
use crate::models::dump::Dump;
use crate::models::id::Id;

use rocket::{
    http::{Method::*, Status},
    route::{Handler, Outcome, Route},
};
use rocket::{Data, Request};

#[derive(Clone)]
struct CustomHandler;

#[rocket::async_trait]
impl Handler for CustomHandler {
    async fn handle<'r>(&self, request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        let id = Id::from(request);
        if id.is_valid() {
            if id.check_fresh().is_ok() {
                let mut dump = Dump::from(request);
                dump.add_data(data).await;

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
}

pub fn app() -> Vec<Route> {
    [Get, Post, Put, Patch, Head, Delete]
        .into_iter()
        .map(|method| Route::ranked(10, method, "/<path..>", CustomHandler))
        .collect()
}
