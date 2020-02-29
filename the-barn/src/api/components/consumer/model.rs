use getset::{CopyGetters, Setters};
use crate::schema::consumers;

#[table_name = "consumers"]
#[derive(CopyGetters, Setters, Serialize, Deserialize, AsChangeset, Default, Queryable, Insertable)]
#[getset(set = "pub")]
pub struct Consumer {
    id: uuid::Uuid,
    name: String,
    address: serde_json::Value,
    description: Option<String>,
    pub markets: Option<Vec<uuid::Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    con_type: Option<i16>
}