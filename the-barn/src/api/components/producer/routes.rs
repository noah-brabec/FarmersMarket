// This file contains all of the endpoints for the producer table
use crate::api::components::producer::model::Producer;
use crate::api::components::producer::controller;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use crate::database::DbConn;

//Returns a list of all producers
#[get("/producers")]
pub fn get_producers(connection : DbConn) -> Json<Vec<Producer>> {
    Json(controller::get_all_prods(connection))
}

/*
#[get("/producers/<radius>")]
fn get_prods_in_radius(radius : i32) -> String {

    return format!("Get poducers within {} miles of you", radius);
}
)*/

#[get("/producers/<id>")]
pub fn get_prod_by_id(id : Uuid, connection : DbConn) -> Json<Producer> {
   Json(controller::get_prod_by_id(id, connection))
}

#[post("/producers", format = "application/json", data = "<producer>")]
pub fn post_producer(producer: Json<Producer>, connection : DbConn) -> String {
    let prod = producer.into_inner();
    controller::construct_producer(prod, connection);
    return "hello".to_string();
}

#[put("/producers", format = "application/json", data = "<body>")]
pub fn update_producer(body : Json<Producer>, connection : DbConn) -> Json<Producer> {
    controller::update_producer(body, connection)
}

#[delete("/producers/<id>")]
pub fn delete_producer(id : Uuid, connection : DbConn) -> Json<String> {
    controller::delete_producer(id, connection)
}