use crate::schema::markets;
use getset::{CopyGetters, Setters};

#[table_name="markets"]
#[derive(Setters, CopyGetters, Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[getset(set = "pub")]
pub struct Market {
    id: uuid::Uuid,
    name: String,
    address: serde_json::Value,
    description: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>
}