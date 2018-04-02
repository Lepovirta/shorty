use std::collections::HashMap;

use shortener::Shortener;


pub trait Repository {
    // Find a stored item
    fn find(&self, id: String) -> Option<&String>;
    // Store an item and return it's ID
    fn store(&mut self, url: &String) -> String;
}

/*
 * This wraps a trait into an object for rocket request handlers in lib.rs.
 * Rust requires concrete types so this is a work around for that.
 * TODO: is a new struct really needed for using a trait object?
 */
pub struct BRepository {
    pub data : Box<Repository + Sync + Send>,
}


/*
 * Example implementation of a Repository using a HashMap.
 */
pub struct InMemoryRepo<T: Shortener> {
    urls: HashMap<String, String>,
    shortener: T,
}

impl<T> InMemoryRepo<T> where T: Shortener {
    pub fn new() -> InMemoryRepo<T> {
        InMemoryRepo {
            urls: HashMap::new(),
            shortener: T::new(),
        }
    }
}

impl<T> Repository for InMemoryRepo<T> where T: Shortener {
    fn find(&self, id: String) -> Option<&String> {
        self.urls.get(&id)
    }

    fn store(&mut self, url: &String) -> String {
        let id = self.shortener.next();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }
}
