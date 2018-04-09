use std::collections::HashMap;
use std::sync::RwLock;

use repository::Repository;

/*
 * Repository implementation using a HashMap. Any Shortener can be used with
 * this Repository as it's generic.
 */
pub struct InMemoryRepo {
    urls: RwLock<HashMap<String, String>>,
}

impl InMemoryRepo {
    pub fn new() -> InMemoryRepo {
        InMemoryRepo {
            urls: RwLock::new(HashMap::new()),
        }
    }
}

impl Repository for InMemoryRepo {
    fn find(&self, id: &str) -> Option<String> {
        let urls = self.urls.read().unwrap();
        urls.get(id).map(|x| x.to_string())
    }

    fn store(&self, id: &str, url: &str) {
        let mut urls = self.urls.write().unwrap();
        urls.insert(id.to_string(), url.to_string());
    }
}
