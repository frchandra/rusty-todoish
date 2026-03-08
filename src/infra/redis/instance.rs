use redis::aio::MultiplexedConnection;

pub async fn open(redis_url: &str) -> MultiplexedConnection {
    match redis::Client::open(redis_url) {

        Ok(redis) => match redis.get_multiplexed_async_connection().await {
            Ok(connection) => {
                eprintln!("Connected to redis");
                connection
            }
            Err(e) => {
                eprintln!("Could not connect to redis: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Could not open redis: {}", e);
            std::process::exit(1);
        }
    }
}
