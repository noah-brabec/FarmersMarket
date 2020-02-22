#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use rocket_contrib::json::Json;

pub mod models;
use models::producer::Producer;

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

