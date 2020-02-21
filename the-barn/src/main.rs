#![feature(proc_macro_hygiene, decl_macro)]

#[marco_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde_derive;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
use models::producer::Producer;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


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

fn main() {
    rocket::ignite().mount("/", routes![index])
                    .mount("/producers", routes![get_producers, post_producer])
                    .launch();
}

