
use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use tracing::info;

pub fn init_consumer() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        //receive channel
        let channel = conn.create_channel().await.expect("create_channel");
        info!(state=?conn.status().state());

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");
        info!(state=?conn.status().state());
        info!(?queue, "Declared queue");

        info!("will consume");
        let mut consumer = channel
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");
        info!(state=?conn.status().state());

        while let Some(delivery) = consumer.next().await {

            let buf_data = &delivery.as_ref().unwrap().data;
            let str_data = std::str::from_utf8(&buf_data).unwrap();
            let json_data = json::parse(str_data).unwrap();

            let id_data: &str = &json_data["id"].to_string();
            let key_data: &str = &json_data["key"].to_string();

            let cache_id = format!("{}{}", id_data, key_data);
            let cache_value = json_data["value"].to_string();
            println!("{:?} {:?}",cache_id,cache_value);
            

            if let Ok(delivery) = delivery {
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    })
}
