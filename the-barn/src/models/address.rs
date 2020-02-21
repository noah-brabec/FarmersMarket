extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct Address {
    street: String,
    city: String,
    state: String,
    zipcode: i32,
    building_num: String
}