extern crate shorty;

use shorty::repository::BRepository;
use shorty::repository::RedisRepo;
use shorty::shortener::HarshShortener;

/*
 * Initialize and launch the app with chosen Repository and Shortener. Here we
 * also need to Box the Repository trait to an trait object.
 */
fn main() {
    let redis_host = std::env::var("REDIS_HOST").unwrap();
    let redis_url  = format!("redis://{}:6379", redis_host);
    let repo: RedisRepo<HarshShortener> = RedisRepo::new(&redis_url, 4);
    let boxed_repo = BRepository { data: Box::new(repo) };
    let app_server = shorty::app(boxed_repo);
    app_server.launch();
}
