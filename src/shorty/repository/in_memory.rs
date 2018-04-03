use std::collections::HashMap;

use repository::Repository;
use shortener::Shortener;


/*
 * Repository implementation using a HashMap. Any Shortener can be used with
 * this Repository as it's generic.
 */
pub struct InMemoryRepo<T: Shortener> {
    urls: HashMap<String, String>,
    shortener: T,
}

impl<T> InMemoryRepo<T> where T: Shortener {
    pub fn new(id_len: usize) -> InMemoryRepo<T> {
        InMemoryRepo {
            urls: HashMap::new(),
            shortener: T::new(id_len),
        }
    }
}

impl<T> Repository for InMemoryRepo<T> where T: Shortener {
    fn find(&self, id: String) -> Option<String> {
        self.urls.get(&id).map(|x| x.to_string())
    }

    fn store(&mut self, url: &String) -> String {
        let id = self.shortener.next();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }
}
