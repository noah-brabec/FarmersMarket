//Receives calls from routes, decides on what service needs to do
use crate::database::DbConn;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use crate::api::components::producer::model::Producer;
use crate::api::components::producer::service;

// Generates a new uuid, assings it to the producer and passes it to service
pub fn construct_producer(mut prod : Producer, connection : DbConn) -> Producer {
    let new_uuid = uuid::Uuid::new_v4();
    prod.set_id(new_uuid);
    service::insert_new_prod(prod, connection)
}

pub fn get_all_prods(connection : DbConn) -> Vec<Producer> {
    service::get_all_prods(connection)
}

pub fn get_prod_by_id(uuid : Uuid, connection : DbConn) -> Producer {
    //This UUID type is from the crate rocket_contrib::uuid and uses the general uuid crate v0.7.4
    let bytes = uuid.as_bytes(); 
    //This UUID type is from the general uuid crate and is on v0.6.5 because of a diesel dependency
    let other_uuid = uuid::Uuid::from_bytes(bytes).expect("Could not convert uuid v0.7.4 to v0.6.5");
    service::get_prod_by_id(other_uuid, connection)
}

pub fn update_producer(body : Json<Producer>, connection : DbConn) -> Json<Producer> {
    Json(service::update_producer(body.into_inner(), connection))
}

pub fn delete_producer(uuid : Uuid, connection : DbConn) -> Json<String> {
    let bytes = uuid.as_bytes();
    let other_uuid = uuid::Uuid::from_bytes(bytes).expect("Could not convert uuid v0.7.4 to v0.6.5");
    let num_deleted = service::delete_producer(other_uuid, connection);
    let response = format!("num_deleted: {}", num_deleted);
    Json(response)
}