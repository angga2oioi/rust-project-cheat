use std::collections::HashMap;

fn main() {
    let mut caches = HashMap::new();

    caches.insert(
        "BANANA".to_string(),
        "12".to_string(),
    );

    caches.insert(
        "APPLE".to_string(),
        "13".to_string(),
    );

    caches.insert(
        "BANANA".to_string(),
        "13".to_string(),
    );
    
    let banana= caches.get(&"BANANA" as &str);

    println!("key: {} val: {:?}","BANANA",banana);

    caches.remove(&"APPLE" as &str);

    let apple= caches.get(&"APPLE" as &str);

    println!("key: {} val: {:?}","APPLE",apple);
}
