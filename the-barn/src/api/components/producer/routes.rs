// This file contains all of the endpoints for the producer table
use crate::api::components::producer::model::Producer;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

//Returns a list of all producers
#[get("/producers")]
fn get_producers() -> Json<String> {
   
   return Json("farms".to_string());
}

#[get("/producers/<radius>")]
fn get_prods_in_radius(radius : i32) -> String {

    return format!("Get poducers within {} miles of you", radius);
}

#[get("/producers/<id>")]
fn get_prod_by_id(id : Uuid) -> String {

    return format!("The uuid is {}", id);
}

#[post("/producers", format = "application/json", data = "<producer>")]
fn post_producer(producer: Json<Producer>) -> String {
    let prod = producer.into_inner();
    println!("{}", prod.name);
    return "hello".to_string();
}