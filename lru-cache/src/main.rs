use lru::LruCache;

mod service;

use crate::service::{
    print_cache
};

fn main() {
    let mut cache : LruCache<&str, &str> = LruCache::new(4);
    cache.put("apple", "12");
    cache.put("jeruk", "13");
    cache.put("pisang", "14");
    cache.put("durian", "15");
    print_cache(&mut cache);
}
