#[macro_use]
extern crate lazy_static;

use json::{object, JsonValue};
use std::collections::HashMap;

use std::sync::{
    Mutex,
};

use std::mem::{
    drop as unlock_mutex
};


use crate::service::print_banana;

mod service;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<&'static str, &'static str>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };    
}


fn main() {
    let mut caches = HASHMAP.lock().unwrap();

    caches.insert("BANANA", "12");

    caches.insert("APPLE", "13");

    caches.insert(
        "BANANA",
        r#"{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}"#
        ,
    );

    caches.remove("APPLE");
    
    let banana = caches.get("BANANA");


    let apple = caches.get("APPLE");

    let apple_result: JsonValue = match apple {
        Some(val) => JsonValue::String(val.to_string()),
        _ => JsonValue::Null,
    };

    let banana_result : JsonValue= match banana {
        Some(val)=>JsonValue::String(val.to_string()),
        _ => JsonValue::Null
    };

    let apple_json = object! {
        id:"APPLE",
        apple:apple_result,
        banana:json::parse(&banana_result.to_string()).unwrap()
    };


    println!("{:?}", json::stringify(apple_json));
    
    unlock_mutex(caches);

    print_banana();

    
}
