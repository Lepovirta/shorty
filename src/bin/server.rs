extern crate shorty;

use shorty::repository::BRepository;
use shorty::repository::InMemoryRepo;
use shorty::shortener::HarshShortener;

/*
 * Initialize and launch the app with chosen Repository and Shortener. Here we
 * also need to Box the Repository trait to an trait object.
 */
fn main() {
    let repo: InMemoryRepo<HarshShortener> = InMemoryRepo::new(4);
    let boxed_repo = BRepository { data: Box::new(repo) };
    let app_server = shorty::app(boxed_repo);
    app_server.launch();
}
