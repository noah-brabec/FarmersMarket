extern crate serde;
extern crate serde_json;

use uuid::Uuid;
use super::geolocation::Geolocation;
use super::address::Address;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct Producer {
    pub id: Option<Uuid>,
    pub name: String,
    pub address: Address,
    pub type_: String,
    pub geolocation: Option<Geolocation>,
    pub markets: Option<Vec<Uuid>>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>
}
