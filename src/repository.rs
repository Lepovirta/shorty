use std::collections::HashMap;
use shortener::HarshShortener;
use shortener::Shortable;

/*
 * Repository stores URLs and provides a shortener.
 */
pub struct Repository<T1, T2> {
    urls: T1,
    shortener: T2,
}

/*
 * Cache is the interface when interacting with a repository.
 */
pub trait Cache {
    // Find a stored item
    fn find(&self, id: String) -> Option<&String>;
    // Store an item and return it's ID
    fn store(&mut self, url: &String) -> String;
}


/*
 * Example implementation of a Repository using a HashMap.
 */
pub type InMemoryRepo = Repository<HashMap<String, String>, HarshShortener>;

impl InMemoryRepo {
    pub fn new() -> InMemoryRepo {
        Repository {
            urls: HashMap::new(),
            shortener: HarshShortener::new(),
        }
    }
}

impl Cache for InMemoryRepo {
    fn find(&self, id: String) -> Option<&String> {
        self.urls.get(&id)
    }

    fn store(&mut self, url: &String) -> String {
        let id = self.shortener.next();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }
}
