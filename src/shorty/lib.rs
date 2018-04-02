#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate harsh;

use std::sync::RwLock;
use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;

use repository::BRepository;

pub mod repository;
pub mod shortener;


#[derive(FromForm)]
struct Url {
    url: String,
}


#[get("/<id>")]
fn find(repo: State<RwLock<BRepository>>, id: String) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().data.find(id) {
        Some(url) => Ok(Redirect::permanent(url)),
        _         => Err("ID not found.")
    }
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<BRepository>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url  = url_form.get().url;
    let mut repo = repo.write().unwrap();
    let id       = repo.data.store(&url);
    Ok(id.to_string())
}

#[get("/")]
fn usage() -> &'static str {
    "
    USAGE\n
      POST /\n
          accepts an URL in the body of the request and responds with an ID\n\n
      GET /<id>\n
          redirects to found url for id `<id>`\n\n
      GET /\n
          Shows this
    "
}


pub fn app(repo: BRepository) -> rocket::Rocket {
    rocket::ignite()
        .manage(RwLock::new(repo))
        .mount("/", routes![find, shorten, usage])
}
