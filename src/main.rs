use std::sync::mpsc::channel;

use redis::{RedisResult, ToRedisArgs};

use crate::data::redis_connector::RedisConnector;

mod data;

fn main() {
    let mut connector = RedisConnector::new(String::from("redis:6379/0"));

    let channel_name = "push_notifications";

    let mut pub_sub = connector.get_pub_sub();
    match pub_sub.subscribe(channel_name) {
        Ok(()) => {
            println!("got pub sub for to {}", channel_name);
            loop {
                let msg = pub_sub.get_message().unwrap();
                let payload: String = msg.get_payload().unwrap();
                println!("channel '{}': {}", msg.get_channel_name(), payload);
            }
        }
        Err(e) => {
            println!("Failed to Subscribe to {}: {}", channel_name, e)
        }
    }
}
