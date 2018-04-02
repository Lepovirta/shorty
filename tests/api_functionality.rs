extern crate rocket;
extern crate shorty;

use rocket::local::Client;
use rocket::http::{ContentType, Status};

use shorty::repository::BRepository;
use shorty::repository::InMemoryRepo;
use shorty::shortener::HarshShortener;


fn test_client() -> rocket::local::Client {
    let repo: InMemoryRepo<HarshShortener> = InMemoryRepo::new();
    let boxed_repo = BRepository { data: Box::new(repo) };
    let rocket     = shorty::app(boxed_repo);
    Client::new(rocket).expect("valid rocket instance")
}


#[test]
fn api_index_reponds_with_ok() {
    let client   = test_client();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn shortening_api_returns_expected_id_for_the_first_url() {
    let client   = test_client();
    let first_id = "gY";
    let request  = client.post("/")
        .header(ContentType::Form)
        .body(r#"url=https://www.rust-lang.org"#);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(first_id.into()));
}

#[test]
fn find_api_returns_an_error_when_requested_with_an_invalid_id() {
    let client   = test_client();
    let response = client.get("/invalid-ID").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn shortening_api_creates_an_working_id_and_redirects_from_find_api_with_it() {
    let client   = test_client();
    let first_id = "gY";
    let request  = client.post("/")
        .header(ContentType::Form)
        .body(r#"url=https://www.rust-lang.org"#);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(first_id.into()));

    let response = client.get(format!("/{}", first_id)).dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
}
