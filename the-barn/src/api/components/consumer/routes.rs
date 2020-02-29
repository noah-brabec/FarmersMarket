// This file contains all of the endpoints for the consumer table
use crate::api::components::consumer::model::Consumer;
use crate::api::components::consumer::controller;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use crate::database::DbConn;

//Returns a list of all consumers
#[get("/consumers")]
pub fn get_consumers(connection : DbConn) -> Json<Vec<Consumer>> {
    Json(controller::get_all_consumers(connection))
}

/*
#[get("/consumers/<radius>")]
fn get_prods_in_radius(radius : i32) -> String {

    return format!("Get poducers within {} miles of you", radius);
}
)*/

#[get("/consumers/<id>")]
pub fn get_consumer_by_id(id : Uuid, connection : DbConn) -> Json<Consumer> {
   Json(controller::get_consumer_by_id(id, connection))
}

#[post("/consumers", format = "application/json", data = "<consumer>")]
pub fn post_consumer(consumer: Json<Consumer>, connection : DbConn) -> String {
    let prod = consumer.into_inner();
    controller::construct_consumer(prod, connection);
    return "hello".to_string();
}

#[put("/consumers", format = "application/json", data = "<body>")]
pub fn update_consumer(body : Json<Consumer>, connection : DbConn) -> Json<Consumer> {
    controller::update_consumer(body, connection)
}

#[delete("/consumers/<id>")]
pub fn delete_consumer(id : Uuid, connection : DbConn) -> Json<String> {
    controller::delete_consumer(id, connection)
}