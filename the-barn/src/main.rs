#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

pub mod api;
pub mod schema;

use rocket_contrib::json::Json;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

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
    rocket::ignite().mount("/", routes![index])
                    .mount("/producers", routes![index])
                    .launch();
}

