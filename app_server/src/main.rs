#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate r2d2_redis;

mod redis;

use diesel::PgConnection;
use r2d2_redis::redis::{Commands, RedisResult};
use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use rocket::{Rocket, State};
use rocket_contrib::serve::StaticFiles;
use shared::settings::Settings;
use std::collections::HashMap;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("app_pg_db")]
struct MyDatabase(PgConnection);

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = MyDatabase::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

#[get("/redis/publish")]
fn redis_publish(mut conn: redis::RedisConnection, config: State<Settings>) {
    dbg!(&config);
    let channel = config.redis.notify_channel.to_owned();
    let _: RedisResult<()> = conn.publish(channel, "messsage");
}

fn main() {
    let settings = Settings::new().expect("error occuered when Settings::new().");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let postgres_url = settings.postgres.url.to_owned();
    let redis_url = settings.redis.url.to_owned();

    database_config.insert("url", Value::from(postgres_url));
    databases.insert("app_pg_db", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .expect("error occuered when insert config.");

    rocket::custom(config)
        .attach(MyDatabase::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .manage(redis::pool(redis_url))
        .manage(settings)
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/pubsub", routes!(redis_publish))
        .launch();
}
