use lru::LruCache;

pub fn print_cache(cache : &mut LruCache<&str, &str>){
    println!("{}",*cache.get(&"apple").unwrap());
    println!("{}",*cache.get(&"jeruk").unwrap());
    println!("{}",*cache.get(&"pisang").unwrap());
    println!("{}",*cache.get(&"durian").unwrap());
}