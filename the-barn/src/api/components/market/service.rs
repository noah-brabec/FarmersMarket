//Reads and writes to the db
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{markets};
use crate::api::components::market::model::Market;
use crate::database::DbConn;

pub fn insert_new_market(market : Market, connection : DbConn) -> Market {
    let db_con = &*connection; 
    diesel::insert_into(markets::table)
        .values(&market)
        .get_result(db_con)
        .expect("Error saving new Market")
}

pub fn get_all_markets(connection : DbConn) -> Vec<Market> {
    let db_con = &*connection;
    markets::table.load::<Market>(db_con).expect("Error reading from markets")
}

pub fn get_market_by_id(uuid : Uuid, connection : DbConn) -> Market {
    let db_con = &*connection;
    markets::table.find(uuid).first(db_con).expect("Could not find Market")
}

pub fn update_market(market : Market, connection : DbConn) -> Market {
    let db_con = &*connection;
    diesel::update(markets::table)
        .set(&market)
        .get_result(db_con)
        .expect("Could not update market")
}

pub fn delete_market(uuid : Uuid, connection : DbConn) -> usize {
    let db_con = &*connection;
    diesel::delete(markets::table.filter(markets::id.eq(uuid)))
        .execute(db_con)
        .expect("Could not delete market")
}