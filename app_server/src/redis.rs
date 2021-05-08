use r2d2_redis::{r2d2, RedisConnectionManager};
use rocket::http;
use rocket::request;
use rocket::Outcome;
use rocket::State;
use std::ops::{Deref, DerefMut};

impl Deref for RedisConnection {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RedisConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct RedisConnection(pub r2d2::PooledConnection<RedisConnectionManager>);

type RedisConnectionPool = r2d2::Pool<RedisConnectionManager>;

pub fn pool(url: String) -> RedisConnectionPool {
    // let url = option_env!("REDIS_URL").unwrap_or("redis://redis_database:56379");
    //  let url = option_env!("REDIS_URL").unwrap_or("redis://localhost:56379");
    let manager = RedisConnectionManager::new(url).expect("connection error happened to redis.");
    r2d2::Pool::builder()
        .build(manager)
        .expect("r2d2 pool build error happened.")
}

impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<RedisConnection, ()> {
        let pool = request.guard::<State<RedisConnectionPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisConnection(conn)),
            Err(_) => Outcome::Failure((http::Status::ServiceUnavailable, ())),
        }
    }
}
