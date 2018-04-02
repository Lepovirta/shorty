extern crate shorty;

use shorty::repository::BRepository;
use shorty::repository::InMemoryRepo;
use shorty::shortener::HarshShortener;

fn main() {
    let repo: InMemoryRepo<HarshShortener> = InMemoryRepo::new();
    let boxed_repo = BRepository { data: Box::new(repo) };
    let app_server = shorty::app(boxed_repo);
    app_server.launch();
}
