extern crate redis;
#[macro_use]
extern crate log;

use redis::RedisError;

#[derive(Debug)]
struct Subscriber {
    redis_url: &'static str,
    channel: &'static str,
}

impl Subscriber {
    fn run(&self) -> Result<(), RedisError> {
        let client = redis::Client::open(self.redis_url)?;
        let mut con = client.get_connection()?;
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe(self.channel)?;

        loop {
            let msg = pubsub.get_message()?;
            let payload: String = msg.get_payload()?;
            info!("channel '{}': {}", msg.get_channel_name(), payload);
        }
    }
}

fn get_redis_url() -> &'static str {
    // option_env!("REDIS_URL").unwrap_or("redis://redis_database:56379/")
    option_env!("REDIS_URL").unwrap_or("redis://localhost:56379/")
}

fn get_channel_name() -> &'static str {
    option_env!("REDIS_CHANNEL").unwrap_or("dev_pubsub_channel")
}

fn main() -> Result<(), RedisError> {
    env_logger::init();
    let url = get_redis_url();
    let channel = get_channel_name();
    info!("🚀 Redis PubSub Subscriber Open: {}", url);
    let subscriber = Subscriber {
        redis_url: url,
        channel,
    };
    info!("Running...");
    subscriber.run()
}
