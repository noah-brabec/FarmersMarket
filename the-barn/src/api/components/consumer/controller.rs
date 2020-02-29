//Receives calls from routes, decides on what service needs to do
use crate::database::DbConn;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use crate::api::components::consumer::model::Consumer;
use crate::api::components::consumer::service;

// Generates a new uuid, assings it to the consumer and passes it to service
pub fn construct_consumer(mut consumer : Consumer, connection : DbConn) -> Consumer {
    let new_uuid = uuid::Uuid::new_v4();
    consumer.set_id(new_uuid);
    service::insert_new_consumer(consumer, connection)
}

pub fn get_all_consumers(connection : DbConn) -> Vec<Consumer> {
    service::get_all_consumers(connection)
}

pub fn get_consumer_by_id(uuid : Uuid, connection : DbConn) -> Consumer {
    //This UUID type is from the crate rocket_contrib::uuid and uses the general uuid crate v0.7.4
    let bytes = uuid.as_bytes(); 
    //This UUID type is from the general uuid crate and is on v0.6.5 because of a diesel dependency
    let other_uuid = uuid::Uuid::from_bytes(bytes).expect("Could not convert uuid v0.7.4 to v0.6.5");
    service::get_consumer_by_id(other_uuid, connection)
}

pub fn update_consumer(body : Json<Consumer>, connection : DbConn) -> Json<Consumer> {
    Json(service::update_consumer(body.into_inner(), connection))
}

pub fn delete_consumer(uuid : Uuid, connection : DbConn) -> Json<String> {
    let bytes = uuid.as_bytes();
    let other_uuid = uuid::Uuid::from_bytes(bytes).expect("Could not convert uuid v0.7.4 to v0.6.5");
    let num_deleted = service::delete_consumer(other_uuid, connection);
    let response = format!("num_deleted: {}", num_deleted);
    Json(response)
}