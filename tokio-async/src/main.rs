use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    
    let test_string : String = String::from("Whatever");

    let handle = tokio::spawn(async move {
        print_fn(test_string).await;
    });

    println!("printed on main");

    handle.await.unwrap();
}

async fn print_fn(test_string:String){
    sleep(Duration::from_secs(4)).await;

    println!("printed here {}",test_string);
}