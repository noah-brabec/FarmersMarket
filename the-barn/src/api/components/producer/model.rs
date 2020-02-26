//The model that follows the db schema
use crate::schema::producers;
use getset::{CopyGetters, Setters};

#[repr(i16)]
#[derive(Serialize, Deserialize)]
pub enum ProducerType {
    GENERIC = 1,
    PLANT = 2,
    COFFEE = 3,
    MEAT = 4,
    MILL = 5
}

#[table_name="producers"]
#[derive(Setters, CopyGetters, Serialize, Deserialize, Queryable, Insertable)]
#[getset(set = "pub")]
pub struct Producer {
    pub id: uuid::Uuid,
    pub name: String,
    address: serde_json::Value,
    markets: Option<Vec<uuid::Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    description: Option<String>,
    prod_type: Option<i16>
}