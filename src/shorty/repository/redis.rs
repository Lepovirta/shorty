use r2d2::Pool as r2d2Pool;
use r2d2_redis::RedisConnectionManager;
use redis::Commands;
use redis::RedisError;

use repository::Repository;
use shortener::Shortener;


/*
 * Repository implementation using Redis. Any Shortener can be used with this
 * Repository as it's generic.
 */

type Pool = r2d2Pool<RedisConnectionManager>;

pub struct RedisRepo<T:Shortener> {
    pool: Pool,
    shortener: T,
}

fn init_pool(db_url: &'static str) -> Pool {
    let manager = RedisConnectionManager::new(db_url).unwrap();
    r2d2Pool::builder()
        .build(manager)
        .unwrap()
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
        let _ : Result<String,RedisError> = conn.set(id.to_string(), url);
        id
    }
}
