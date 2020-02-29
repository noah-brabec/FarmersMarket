#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

pub mod api;
pub mod schema;
pub mod database;

use api::components::producer::routes as prod_routes;
use api::components::consumer::routes as cons_routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().manage(database::init_pool())
                    .mount("/", routes![index,
                                        prod_routes::post_producer,
                                        prod_routes::get_producers,
                                        prod_routes::get_prod_by_id,
                                        prod_routes::delete_producer,
                                        cons_routes::post_consumer,
                                        cons_routes::get_consumers,
                                        cons_routes::get_consumer_by_id,
                                        cons_routes::delete_consumer])
                    .launch();
}

