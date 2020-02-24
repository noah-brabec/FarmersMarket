//The model that follows the db schema
use crate::schema::producers;

#[derive(Serialize, Deserialize)]
pub enum ProducerType {
    GENERIC,
    PLANT,
    COFFEE,
    MEAT,
    MILL
}

#[table_name="producers"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Producer {
    id: uuid::Uuid,
    pub name: String,
    address: serde_json::Value,
    type_: String,
    markets: Option<Vec<uuid::Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    description: Option<String>
}
