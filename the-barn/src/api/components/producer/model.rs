//The model that follows the db schema
use diesel::pg::types::sql_types::Json;
use crate::api::components::address::model::Address;
use crate::schema::producers;

#[derive(Serialize, Deserialize)]
pub enum ProducerType {
    GENERIC,
    PLANT,
    COFFEE,
    MEAT,
    MILL
}

#[table_name="producers"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Producer {
    id: uuid::Uuid,
    pub name: String,
    address: Json,
    type_: String,
    markets: Option<Vec<uuid::Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    description: Option<String>
}
