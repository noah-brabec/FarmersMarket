//Reads and writes to the db
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use crate::schema::{producers};
use crate::api::components::producer::model::Producer;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_new_prod(prod : Producer) -> Producer {
    let db_con = establish_connection();
    diesel::insert_into(producers::table)
        .values(&prod)
        .get_result(&db_con)
        .expect("Error saving new Producer")
}

pub fn get_all_prods() -> Vec<Producer> {
    let db_con = establish_connection();
    producers::table.load::<Producer>(&db_con).expect("Error reading from producers")
}

pub fn get_prod_by_id(uuid : Uuid) -> Producer {
    let db_con = establish_connection();
    producers::table.find(uuid).first(&db_con).expect("Could not find Producer")
}

pub fn update_producer(producer : Producer) -> Producer {
    let db_con = establish_connection();
    diesel::update(producers::table)
        .set(&producer)
        .get_result(&db_con)
        .expect("Could not update producer")
}