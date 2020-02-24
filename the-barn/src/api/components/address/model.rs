extern crate serde;

use crate::api::components::address::geolocation::Geolocation;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct Address {
    street: String,
    city: String,
    state: String,
    zipcode: i32,
    building_num: String,
    geolocation: Option<Geolocation>
}