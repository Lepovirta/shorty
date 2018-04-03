use std::collections::HashMap;

use shortener::Shortener;


/*
 * Defines common functions for implementing a Repository.
 * find: return stored String for given ID
 * store: store given String and return an ID for it
 */
pub trait Repository {
    fn find(&self, id: String) -> Option<String>;
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
    fn find(&self, id: String) -> Option<String> {
        self.urls.get(&id).map(|x| x.to_string())
    }

    fn store(&mut self, url: &String) -> String {
        let id = self.shortener.next();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }
}




extern crate r2d2;
extern crate redis;
use r2d2_redis::RedisConnectionManager;
use redis::Commands;


type Pool = r2d2::Pool<RedisConnectionManager>;

fn init_pool(db_url: &'static str) -> Pool {
    let manager = RedisConnectionManager::new(db_url).unwrap();
    r2d2::Pool::builder()
        .build(manager)
        .unwrap()
}

pub struct RedisRepo<T:Shortener> {
    pool: Pool,
    shortener: T,
}

impl<T> RedisRepo<T> where T: Shortener {
    pub fn new(db_url: &'static str, id_len: usize) -> RedisRepo<T> {
        RedisRepo {
            pool: init_pool(db_url),
            shortener : T::new(id_len),
        }
    }
}

impl<T> Repository for RedisRepo<T> where T: Shortener {
    fn find(&self, id: String) -> Option<String> {
        let conn = self.pool.get().unwrap();
        match conn.get(id) {
            Ok(r) => Some(r),
            Err(_) => None
        }
    }

    fn store(&mut self, url: &String) -> String {
        let id   = self.shortener.next();
        let conn = self.pool.get().unwrap();
        let _ : Result<String,redis::RedisError> = conn.set(id.to_string(), url);
        id
    }
}
