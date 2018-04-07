extern crate shorty;

use shorty::repository::BRepository;
use shorty::repository::RedisRepo;
use shorty::shortener::Shortener;
use shorty::shortener::HarshShortener;


/*
 * Format the Redis URL and initialize a Redis repository with Shortener.
 */
fn build_redis_repo<T>() -> RedisRepo<T> where T: Shortener {
    let redis_host = std::env::var("REDIS_HOST").unwrap();
    let redis_port = std::env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let redis_url  = format!("redis://{}:{}", redis_host, redis_port);
    RedisRepo::new(&redis_url, 4)
}

/*
 * Initialize and launch the app with chosen Repository and Shortener. Here we
 * also need to Box the Repository trait to an trait object.
 */
fn main() {
    let repo       = build_redis_repo::<HarshShortener>();
    let boxed_repo = BRepository { data: Box::new(repo) };
    let app_server = shorty::app(boxed_repo);
    app_server.launch();
}
