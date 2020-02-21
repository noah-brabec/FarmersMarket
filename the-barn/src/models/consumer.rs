extern crate serde;
extern crate serde_json;

use uuid::Uuid;
use crate::geolocation::Geolocation;
use crate::address::Address;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct Consumer {
    id: UUID,
    name: String,
    address: Address,
    type: String,
    geolocation: Geolocation,
    description: Nullable<String>,
    pub markets: Option<Vec<Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>
}