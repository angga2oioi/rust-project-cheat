use json::{object, JsonValue};
use std::collections::HashMap;

fn main() {
    let mut caches = HashMap::new();

    caches.insert("BANANA".to_string(), "12".to_string());

    caches.insert("APPLE".to_string(), "13".to_string());

    caches.insert(
        "BANANA".to_string(),
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
        .to_string(),
    );

    let banana = caches.get(&"BANANA" as &str).unwrap().to_owned();

    caches.remove(&"APPLE" as &str);

    let apple = caches.get(&"APPLE" as &str);

    let apple_result: JsonValue = match apple {
        Some(val) => JsonValue::String(val.to_string()),
        _ => JsonValue::Null,
    };

    let apple_json = object! {
        id:"APPLE",
        result:apple_result,
        banana:json::parse(&banana).unwrap()
    };


    println!("{:?}", json::stringify(apple_json));
}
