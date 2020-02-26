//Receives calls from routes, decides on what service needs to do
use rocket_contrib::uuid::Uuid;
use crate::api::components::producer::model::Producer;
use crate::api::components::producer::service;

// Generates a new uuid, assings it to the producer and passes it to service
pub fn assign_uuid(mut prod : Producer) -> Producer {
    let new_uuid = uuid::Uuid::new_v4();
    prod.set_id(new_uuid);
    service::insert_new_prod(prod)
}

pub fn get_all_prods() -> Vec<Producer> {
    service::get_all_prods()
}

pub fn get_prod_by_id(uuid : Uuid) -> Producer {
    let bytes = uuid.as_bytes();
    let other_uuid = uuid::Uuid::from_bytes(bytes).expect("Could not convert uuid v0.7.4 to v0.6.5");
    service::get_prod_by_id(other_uuid)
}