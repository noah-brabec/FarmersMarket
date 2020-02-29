// This file contains all of the endpoints for the market table
use crate::api::components::market::model::Market;
use crate::api::components::market::controller;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use crate::database::DbConn;

//Returns a list of all markets
#[get("/markets")]
pub fn get_markets(connection : DbConn) -> Json<Vec<Market>> {
    Json(controller::get_all_markets(connection))
}

/*
#[get("/markets/<radius>")]
fn get_prods_in_radius(radius : i32) -> String {

    return format!("Get poducers within {} miles of you", radius);
}
)*/

#[get("/markets/<id>")]
pub fn get_market_by_id(id : Uuid, connection : DbConn) -> Json<Market> {
   Json(controller::get_market_by_id(id, connection))
}

#[post("/markets", format = "application/json", data = "<market>")]
pub fn post_market(market: Json<Market>, connection : DbConn) -> String {
    let prod = market.into_inner();
    controller::construct_market(prod, connection);
    return "hello".to_string();
}

#[put("/markets", format = "application/json", data = "<body>")]
pub fn update_market(body : Json<Market>, connection : DbConn) -> Json<Market> {
    controller::update_market(body, connection)
}

#[delete("/markets/<id>")]
pub fn delete_market(id : Uuid, connection : DbConn) -> Json<String> {
    controller::delete_market(id, connection)
}