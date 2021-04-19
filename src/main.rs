use crate::redis_client::redis_connector::RedisConnector;

mod redis_client;
mod firebase;

fn main() {
    println!("Connecting to redis");
    let mut connector = RedisConnector::new(String::from("redis:6379/0"));

    let channel_name = "push_notifications";

    let mut pub_sub = connector.get_pub_sub();
    match pub_sub.subscribe(channel_name) {
        Ok(()) => {
            println!("Connected");
            println!("got pub sub for to {}", channel_name);
            loop {
                let msg = pub_sub.get_message().unwrap();
                let payload: String = msg.get_payload().unwrap();
            }
        }
        Err(e) => {
            println!("Failed to Subscribe to {}: {}", channel_name, e)
        }
    }
}
