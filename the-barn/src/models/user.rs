extern crate serde;
extern crate serde_json;

use uuid::Uuid;
use crate::geolocation::Geolocation;
use crate::address::Address;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct User {
    id: UUID,
    first: String,
    last: String,
    email: String,
    phone: Option<String>,
    password: String
}