use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    
    let handle = tokio::spawn(async move {
        print_fn().await;
    });

    println!("printed on main");

    handle.await.unwrap();
}

async fn print_fn(){
    sleep(Duration::from_secs(4)).await;

    println!("printed here");
}