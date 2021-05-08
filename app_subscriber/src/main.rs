extern crate redis;
#[macro_use]
extern crate log;

use redis::RedisError;
use shared::settings::Settings;

#[derive(Debug)]
struct Subscriber {
    redis_url: String,
    channel: String,
}

impl Subscriber {
    fn run(&self) -> Result<(), RedisError> {
        let client = redis::Client::open(self.redis_url.as_str())?;
        let mut con = client.get_connection()?;
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe(self.channel.as_str())?;

        loop {
            let msg = pubsub.get_message()?;
            let payload: String = msg.get_payload()?;
            info!("channel '{}': {}", msg.get_channel_name(), payload);
        }
    }
}

fn main() -> Result<(), RedisError> {
    env_logger::init();
    let settings = Settings::new().expect("error occuered when Settings::new().");
    let redis = settings.redis.to_owned();
    let url = redis.url.to_owned();
    let channel = redis.notify_channel.to_owned();
    info!("🚀 Redis PubSub Subscriber Open: {}", url);
    let subscriber = Subscriber {
        redis_url: url,
        channel,
    };
    info!("Running...");
    subscriber.run()
}
