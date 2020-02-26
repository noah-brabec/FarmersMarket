// This file contains all of the endpoints for the producer table
use crate::api::components::producer::model::Producer;
use crate::api::components::producer::controller;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

//Returns a list of all producers
#[get("/producers")]
pub fn get_producers() -> Json<Vec<Producer>> {
    Json(controller::get_all_prods())
}

#[get("/producers/<radius>")]
fn get_prods_in_radius(radius : i32) -> String {

    return format!("Get poducers within {} miles of you", radius);
}

#[get("/producers/<id>")]
pub fn get_prod_by_id(id : Uuid) -> Json<Producer> {
   Json(controller::get_prod_by_id(id))
}

#[post("/producers", format = "application/json", data = "<producer>")]
pub fn post_producer(producer: Json<Producer>) -> String {
    let prod = producer.into_inner();
    controller::construct_producer(prod);
    return "hello".to_string();
}

#[put("/producers", format = "application/json", data = "<body>")]
pub fn update_producer(body : Json<Producer>) -> Json<Producer> {
    controller::update_producer(body)
}