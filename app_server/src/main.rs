#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use diesel::PgConnection;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::databases::redis;
use rocket_contrib::serve::StaticFiles;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("app_pg_db")]
struct MyDatabase(PgConnection);

#[database("app_redis_db")]
struct RedisDatabase(redis::Connection);

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

fn main() {
    rocket::ignite()
        .attach(MyDatabase::fairing())
        .attach(RedisDatabase::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
