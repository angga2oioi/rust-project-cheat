#![deny(warnings)]

use futures_util::{ StreamExt,SinkExt};
use warp::Filter;
use json::{object};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path!()
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| ws_handler(websocket))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 6001)).await;
}

async fn ws_handler(websocket: warp::ws::WebSocket){
    // Just echo all messages back...
    let (mut tx, mut rx) = websocket.split();
    
    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };
        let raw_msg = msg.to_str().unwrap();
        let json_msg = json::parse(raw_msg).unwrap();

        let response = object!{
            "jsonrpc": "2.0",
            "result": 43,
            "id":json_msg["id"].as_str()
        };

        println!("msg: {:?}", msg);
        println!("response: {}", json_msg.to_string());

        let _ = tx.send(warp::ws::Message::text(response.to_string())).await;

    }

    
}