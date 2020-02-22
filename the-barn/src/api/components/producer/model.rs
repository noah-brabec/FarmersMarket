use uuid::Uuid;
use crate::api::components::address::geolocation::Geolocation;
use crate::api::components::address::model::Address;

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
