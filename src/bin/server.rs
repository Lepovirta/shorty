extern crate shorty;

use shorty::repository::RedisRepo;
use shorty::shortener::HarshShortener;


/*
 * Format the Redis URL and initialize a Redis repository with Shortener.
 */
fn build_redis_repo() -> RedisRepo {
    let redis_host = std::env::var("REDIS_HOST").unwrap();
    let redis_port = std::env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let redis_url  = format!("redis://{}:{}", redis_host, redis_port);
    RedisRepo::new(&redis_url)
}

/*
 * Initialize and launch the app with chosen Repository and Shortener. Here we
 * also need to Box the Repository trait to an trait object.
 */
fn main() {
    let repo       = build_redis_repo();
    let shortener  = HarshShortener::new(4);
    let app_server = shorty::app(repo, shortener);
    app_server.launch();
}
