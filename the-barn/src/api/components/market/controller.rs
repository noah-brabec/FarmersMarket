//Receives calls from routes, decides on what service needs to do
use crate::database::DbConn;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use crate::api::components::market::model::Market;
use crate::api::components::market::service;

// Generates a new uuid, assings it to the market and passes it to service
pub fn construct_market(mut market : Market, connection : DbConn) -> Market {
    let new_uuid = uuid::Uuid::new_v4();
    prod.set_id(new_uuid);
    service::insert_new_market(market, connection)
}

pub fn get_all_markets(connection : DbConn) -> Vec<Market> {
    service::get_all_markets(connection)
}

pub fn get_market_by_id(uuid : Uuid, connection : DbConn) -> Market {
    //This UUID type is from the crate rocket_contrib::uuid and uses the general uuid crate v0.7.4
    let bytes = uuid.as_bytes(); 
    //This UUID type is from the general uuid crate and is on v0.6.5 because of a diesel dependency
    let other_uuid = uuid::Uuid::from_bytes(bytes).expect("Could not convert uuid v0.7.4 to v0.6.5");
    service::get_market_by_id(other_uuid, connection)
}

pub fn update_market(body : Json<Market>, connection : DbConn) -> Json<Market> {
    Json(service::update_market(body.into_inner(), connection))
}

pub fn delete_market(uuid : Uuid, connection : DbConn) -> Json<String> {
    let bytes = uuid.as_bytes();
    let other_uuid = uuid::Uuid::from_bytes(bytes).expect("Could not convert uuid v0.7.4 to v0.6.5");
    let num_deleted = service::delete_market(other_uuid, connection);
    let response = format!("num_deleted: {}", num_deleted);
    Json(response)
}