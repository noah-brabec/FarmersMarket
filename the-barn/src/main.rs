#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

pub mod api;
pub mod schema;
pub mod database;

use api::components::producer::routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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

