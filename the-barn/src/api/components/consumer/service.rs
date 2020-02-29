use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{consumers};
use crate::api::components::consumer::model::Consumer;
use crate::database::DbConn;

pub fn insert_new_consumer(consumer : Consumer, connection : DbConn) -> Consumer {
    let db_con = &*connection; 
    diesel::insert_into(consumers::table)
        .values(&consumer)
        .get_result(db_con)
        .expect("Error saving new Producer")
}

pub fn get_all_consumers(connection : DbConn) -> Vec<Consumer> {
    let db_con = &*connection;
    consumers::table.load::<Consumer>(db_con).expect("Error reading from consumers")
}

pub fn get_consumer_by_id(uuid : Uuid, connection : DbConn) -> Consumer {
    let db_con = &*connection;
    consumers::table.find(uuid).first(db_con).expect("Could not find Consumer")
}

pub fn update_consumer(consumer : Consumer, connection : DbConn) -> Consumer {
    let db_con = &*connection;
    diesel::update(consumers::table)
        .set(&consumer)
        .get_result(db_con)
        .expect("Could not update consumer")
}

pub fn delete_consumer(uuid : Uuid, connection : DbConn) -> usize {
    let db_con = &*connection;
    diesel::delete(consumers::table.filter(consumers::id.eq(uuid)))
        .execute(db_con)
        .expect("Could not delete consumer")
}