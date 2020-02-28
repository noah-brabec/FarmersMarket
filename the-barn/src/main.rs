#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

pub mod api;
pub mod database;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use api::components::producer::routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    rocket::ignite().manage(database::init_pool())
                    .mount("/", routes![index,
                                        routes::post_producer,
                                        routes::get_producers,
                                        routes::get_prod_by_id,
                                        routes::delete_producer])
                    .launch();
}

