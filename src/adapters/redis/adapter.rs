use redis::Client;

pub fn new_redis_adapter(url: String) -> Client {
    redis::Client::open(url.clone())
        .unwrap_or_else(|_| panic!("Error connecting to redis database: {}", url))
}
