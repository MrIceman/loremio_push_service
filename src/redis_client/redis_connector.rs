use redis::{Commands, Connection, FromRedisValue, ToRedisArgs, PubSub};

pub struct RedisConnector {
    pub connection: redis::Connection,
}

fn connect(host: String) -> redis::RedisResult<Connection> {
    let mut redis_host = String::from("redis://");
    redis_host.push_str(&host);
    print!("Connecting to {}", redis_host);
    let client = &redis::Client::open(redis_host)?;

    let con = client.get_connection()?;
    Ok(con)
}

impl RedisConnector {
    pub fn new<'a>(host: String) -> RedisConnector {
        let connection = connect(host);
        match connection {
            Ok(result) => { RedisConnector { connection: result } }
            Err(err) => {
                panic!("{}",err.to_string());
            }
        }
    }

    pub fn set<T: ToRedisArgs>(&mut self, key: &str, value: T) -> redis::RedisResult<()> {
        let _: () = self.connection.set(key, value)?;
        Ok(())
    }

    pub fn get<T: FromRedisValue>(&mut self, key: &str) -> redis::RedisResult<T> {
        self.connection.get(key)
    }

    pub fn get_pub_sub(&mut self) -> PubSub {
        self.connection.as_pubsub()
    }
}
