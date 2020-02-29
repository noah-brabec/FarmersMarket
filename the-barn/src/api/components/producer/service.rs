//Reads and writes to the db
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{producers};
use crate::api::components::producer::model::Producer;
use crate::database::DbConn;

pub fn insert_new_prod(prod : Producer, connection : DbConn) -> Producer {
    let db_con = &*connection; 
    diesel::insert_into(producers::table)
        .values(&prod)
        .get_result(db_con)
        .expect("Error saving new Producer")
}

pub fn get_all_prods(connection : DbConn) -> Vec<Producer> {
    let db_con = &*connection;
    producers::table.load::<Producer>(db_con).expect("Error reading from producers")
}

pub fn get_prod_by_id(uuid : Uuid, connection : DbConn) -> Producer {
    let db_con = &*connection;
    producers::table.find(uuid).first(db_con).expect("Could not find Producer")
}

pub fn update_producer(producer : Producer, connection : DbConn) -> Producer {
    let db_con = &*connection;
    diesel::update(producers::table)
        .set(&producer)
        .get_result(db_con)
        .expect("Could not update producer")
}

pub fn delete_producer(uuid : Uuid, connection : DbConn) -> usize {
    let db_con = &*connection;
    diesel::delete(producers::table.filter(producers::id.eq(uuid)))
        .execute(db_con)
        .expect("Could not delete producer")
}