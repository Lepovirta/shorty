extern crate shorty;

use shorty::repository::InMemoryRepo;
use shorty::shortener::HarshShortener;

fn main() {
    shorty::app::<InMemoryRepo<HarshShortener>>().launch();
}
