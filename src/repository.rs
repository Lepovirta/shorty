use std::collections::HashMap;

use shortener::Shortener;


pub trait Cache {
    fn new() -> Self;
    // Find a stored item
    fn find(&self, id: String) -> Option<&String>;
    // Store an item and return it's ID
    fn store(&mut self, url: &String) -> String;
}


/*
 * Example implementation of a Repository using a HashMap.
 */
pub struct InMemoryRepo<S: Shortener> {
    urls: HashMap<String, String>,
    shortener: S,
}

impl<S> Cache for InMemoryRepo<S> where S: Shortener {
    fn new() -> InMemoryRepo<S> {
        InMemoryRepo {
            urls: HashMap::new(),
            shortener: S::new(),
        }
    }
    fn find(&self, id: String) -> Option<&String> {
        self.urls.get(&id)
    }

    fn store(&mut self, url: &String) -> String {
        let id = self.shortener.next();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }
}
