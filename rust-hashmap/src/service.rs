use crate::HASHMAP;

pub fn print_banana(){

    let mut caches = HASHMAP.lock().unwrap();
    
    caches.insert("BANANA","PEEL");

    let banana = caches.get("BANANA").unwrap();

    println!("banana is :{}",banana)
}