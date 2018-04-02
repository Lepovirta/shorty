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
fn it_works() {
    let client   = test_client();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn shortening_api() {
    let client   = test_client();
    let request  = client.post("/")
        .header(ContentType::Form)
        .body(r#"url=https://www.rust-lang.org"#);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("gY".into()));
}

#[test]
fn find_api_error() {
    let client   = test_client();
    let response = client.get("/non-existing-ID").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}


#[test]
fn shortening_and_finding_api_flow() {
    let client   = test_client();
    let first_id = "gY";

    let request = client.post("/")
        .header(ContentType::Form)
        .body(r#"url=https://www.rust-lang.org"#);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(first_id.into()));

    let response = client.get(format!("/{}", first_id)).dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
}
