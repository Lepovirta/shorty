#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate harsh;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::sync::Arc;
use std::sync::RwLock;
use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;

use repository::Repository;
use shortener::Shortener;

pub mod repository;
pub mod shortener;

struct AppState {
    repo: Arc<Repository>,
    shortener: Arc<RwLock<Shortener>>,
}

impl AppState {
    fn new<R, S>(repo: R, shortener: S) -> AppState
        where R: Repository + 'static, S: Shortener + 'static {
        AppState {
            repo: Arc::new(repo),
            shortener: Arc::new(RwLock::new(shortener)),
        }
    }
}

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
fn find(state: State<AppState>, id: String) -> Option<Redirect> {
    state.repo
        .find(&id)
        .map(|url| Redirect::permanent(&url))
}

/*
 * Get an URL from request form data.
 * Respond with a short ID for valid data and error for the rest..
 */
#[post("/", data = "<url_form>")]
fn shorten(state: State<AppState>, url_form: Form<Url>) -> Result<String, String> {
    let ref url  = url_form.get().url;
    let id       = state.shortener.write().unwrap().next();
    state.repo.store(&id, &url);
    Ok(id)
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
pub fn app<R, S>(repo: R, shortener: S) -> rocket::Rocket
    where R: Repository + 'static, S: Shortener + 'static {
    rocket::ignite()
        .manage(AppState::new(repo, shortener))
        .mount("/", routes![find, shorten, usage])
}
