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
use crate::api::components::producer::model::Producer;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/producers")]
fn get_producers() -> Json<String> {
   
   return Json("farms".to_string());
}

#[post("/producers", format = "application/json", data = "<producer>")]
fn post_producer(producer: Json<Producer>) -> String {
    let prod = producer.into_inner();
    println!("{}", prod.name);
    return "hello".to_string();
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
                    .mount("/producers", routes![get_producers, post_producer])
                    .launch();
}

