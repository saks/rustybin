extern crate rocket;
extern crate rustybin;

use rocket::http::Status;
use rustybin::models::bin::Bin;
mod common;

#[test]
fn root_page_is_accesible() {
    common::reset_db();
    let client = common::client();

    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn root_page_has_create_button() {
    common::reset_db();
    let client = common::client();

    let mut response = client.get("/").dispatch();

    let body = response.body_string().unwrap();
    assert!(body.contains("create new bin"));
}

#[test]
fn it_should_redirect() {
    common::reset_db();
    let client = common::client();

    let response = client.post("/").dispatch();

    assert_eq!(response.status(), Status::SeeOther);
    assert!(response.headers().contains("Location"));
}

#[test]
fn it_should_redirect_to_the_new_bin_page() {
    common::reset_db();
    let client = common::client();

    let response = client.post("/").dispatch();
    let headers = response.headers();
    let location = headers.get("Location").next().unwrap();

    let last_bin = get_last_bin().unwrap();
    let expected_url = format!("/__rustybin/{}", last_bin.id);

    assert_eq!(expected_url, location);
}

fn get_last_bin() -> Option<Bin> {
    let mut all_bins = Bin::all().unwrap();
    all_bins.pop()
}

#[test]
fn it_should_create_only_one_new_bin() {
    common::reset_db();
    let client = common::client();

    assert_eq!(0, Bin::all().unwrap().len());
    client.post("/").dispatch();
    assert_eq!(1, Bin::all().unwrap().len());
}

mod capturing {
    use super::*;
    use rocket::local::LocalResponse;

    #[test]
    fn it_should_accept_get() {
        common::reset_db();
        let client = common::client();
        let response = client.get(new_url()).dispatch();

        assert_response_is_success(response);
    }

    #[test]
    fn it_should_accept_post() {
        common::reset_db();
        let client = common::client();
        let response = client.post(new_url()).dispatch();

        assert_response_is_success(response);
    }

    fn new_url() -> String {
        let bin = Bin::create().unwrap();
        format!("/{}", bin.id)
    }

    fn assert_response_is_success(response: LocalResponse) {
        let mut response = response;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(Some("OK".into()), response.body_string());
    }
}
