extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct Geolocation {
    longitude: f32,
    latitude: f32
}