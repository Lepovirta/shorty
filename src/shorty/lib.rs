#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate harsh;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::sync::RwLock;
use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;

use repository::BRepository;

pub mod repository;
pub mod shortener;

/*
 * Struct for defining the form data content.
 */
#[derive(FromForm)]
struct Url {
    url: String,
}


/*
 * Try to find URL for the requested id from the Repository.
 * Redirect to the found URL or return 404 error.
 */
#[get("/<id>")]
fn find(brepo: State<RwLock<BRepository>>, id: String) -> Option<Redirect> {
    let repo = &brepo.read().unwrap();
    repo.data
        .find(id)
        .map(|url| Redirect::permanent(&url))
}

/*
 * Get an URL from request form data.
 * Respond with a short ID for valid data and error for the rest..
 */
#[post("/", data = "<url_form>")]
fn shorten(brepo: State<RwLock<BRepository>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url  = url_form.get().url;
    let mut repo = brepo.write().unwrap();
    let id       = repo.data.store(&url);
    Ok(id.to_string())
}

/*
 * Return usage help for the API.
 */
#[get("/")]
fn usage() -> &'static str {
    "
    USAGE\n
      POST /\n
          accepts an URL as form data ('url=example.com') and responds with an ID\n\n
      GET /<id>\n
          redirects to found url for id `<id>`\n\n
      GET /\n
          Shows this
    "
}


/*
 * Build and return an app ready to be launched.
 */
pub fn app(repo: BRepository) -> rocket::Rocket {
    rocket::ignite()
        .manage(RwLock::new(repo))
        .mount("/", routes![find, shorten, usage])
}
