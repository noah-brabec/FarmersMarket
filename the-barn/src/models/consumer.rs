use uuid::Uuid;
use super::geolocation::Geolocation;
use super::address::Address;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct Consumer {
    id: Uuid,
    name: String,
    address: Address,
    type_: String,
    geolocation: Geolocation,
    description: Option<String>,
    pub markets: Option<Vec<Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>
}