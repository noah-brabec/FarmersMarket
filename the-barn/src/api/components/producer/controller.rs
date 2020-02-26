//Receives calls from routes, decides on what service needs to do
use uuid::Uuid;
use crate::api::components::producer::model::Producer;
use crate::api::components::producer::service;

// Generates a new uuid, assings it to the producer and passes it to service
pub fn assign_uuid(mut prod : Producer) -> Producer {
    let new_uuid = Uuid::new_v4();
    prod.set_id(new_uuid);
    service::insert_new_prod(prod)
}

pub fn get_all_prods() -> Vec<Producer> {
    service::get_all_prods()
}