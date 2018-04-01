#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate harsh;

use std::sync::RwLock;
use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;

mod repository;
use repository::InMemoryRepo;
use repository::Cache;
mod shortener;
use shortener::HarshShortener;


#[derive(FromForm)]
struct Url {
    url: String,
}

/*
 * fn find<C: Cache + Sync + Send>(repo: State<RwLock<C>>, id: String) -> Result<Redirect, &'static str> {
 * error[E0412]: cannot find type `C` in this scope
 */
#[get("/<id>")]
fn find(repo: State<RwLock<InMemoryRepo<HarshShortener>>>, id: String) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().find(id) {
        Some(url) => Ok(Redirect::permanent(url)),
        _         => Err("ID not found.")
    }
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<InMemoryRepo<HarshShortener>>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url  = url_form.get().url;
    let mut repo = repo.write().unwrap();
    let id       = repo.store(&url);
    Ok(id.to_string())
}

#[get("/")]
fn usage() -> &'static str {
    "
    USAGE

      POST /

          accepts an URL in the body of the request and responds with an ID


      GET /<id>

          redirects to found url for id `<id>`


      GET /

          Shows this
    "
}


fn run<C>() where
    C : Cache + Sync + Send + 'static {
    rocket::ignite()
        .manage(RwLock::new(C::new()))
        .mount("/", routes![find, shorten, usage])
        .launch();
}

fn main() {
    run::<InMemoryRepo<HarshShortener>>()
}
