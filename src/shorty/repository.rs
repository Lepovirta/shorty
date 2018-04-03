use std::collections::HashMap;

use shortener::Shortener;


/*
 * Defines common functions for implementing a Repository.
 * find: return stored String for given ID
 * store: store given String and return an ID for it
 */
pub trait Repository {
    fn find(&self, id: String) -> Option<&String>;
    fn store(&mut self, url: &String) -> String;
}

/*
 * Wrap Repository inside a Box making it an trait object. This way it is
 * possible to pass it in rocket library's request handlers. Request handlers
 * require concrete types and this can be achieved with a trait object
 * providing dynamic dispatch.
 * TODO: is this the 'best' way?
 */
pub struct BRepository {
    pub data : Box<Repository + Sync + Send>,
}


/*
 * Example implementation of a Repository using a HashMap. Any Shortener can
 * be used with this Repository as it's generic.
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
    fn find(&self, id: String) -> Option<&String> {
        self.urls.get(&id)
    }

    fn store(&mut self, url: &String) -> String {
        let id = self.shortener.next();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }
}
