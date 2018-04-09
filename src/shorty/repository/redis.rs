use r2d2::Pool as r2d2Pool;
use r2d2_redis::RedisConnectionManager;
use redis::Commands;
use redis::RedisError;

use repository::Repository;


/*
 * Repository implementation using Redis. Any Shortener can be used with this
 * Repository as it's generic.
 */

type Pool = r2d2Pool<RedisConnectionManager>;

pub struct RedisRepo {
    pool: Pool,
}

fn init_pool(db_url: &str) -> Pool {
    let manager = RedisConnectionManager::new(db_url).unwrap();
    r2d2Pool::builder()
        .build(manager)
        .unwrap()
}

impl RedisRepo {
    pub fn new(db_url: &str) -> RedisRepo {
        RedisRepo {
            pool: init_pool(db_url),
        }
    }
}

impl Repository for RedisRepo {
    fn find(&self, id: &str) -> Option<String> {
        let conn = self.pool.get().unwrap();
        match conn.get(id) {
            Ok(r) => Some(r),
            Err(_) => None
        }
    }

    fn store(&self, id: &str, url: &str) {
        let conn = self.pool.get().unwrap();
        let _ : Result<String, RedisError> = conn.set(id.to_string(), url);
    }
}
