//The model that follows the db schema

use crate::api::components::address::geolocation::Geolocation;
use crate::api::components::address::model::Address;

#[derive(Serialize, Deserialize)]
pub enum ProducerType {
    GENERIC,
    PLANT,
    COFFEE,
    MEAT,
    MILL
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Producer {
    id: uuid::Uuid,
    pub name: String,
    address: Address,
    type_: String,
    markets: Option<Vec<uuid::Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    description: Option<String>
}
