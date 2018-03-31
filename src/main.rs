#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate harsh;

use std::sync::RwLock;
use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;

mod repository;
mod shortener;
use repository::InMemoryRepo;
use repository::Cache;


#[derive(FromForm)]
struct Url {
    url: String,
}

#[get("/<id>")]
fn find(repo: State<RwLock<InMemoryRepo>>, id: String) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().find(id) {
        Some(url) => Ok(Redirect::permanent(url)),
        _         => Err("ID not found.")
    }
    //format!("You asked for: {}", id)
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<InMemoryRepo>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url  = url_form.get().url;
    let mut repo = repo.write().unwrap();
    let id       = repo.store(&url);
    Ok(id.to_string())
    //format!("We shortened '{}' for you.", url)
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


fn main() {
    rocket::ignite()
        .manage(RwLock::new(InMemoryRepo::new()))
        .mount("/", routes![find, shorten, usage])
        .launch();
}
