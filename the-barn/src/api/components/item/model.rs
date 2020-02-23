use uuid::Uuid;
use crate::schema::items

#[table_name="items"]
#[derive(Serialize, Deserialize, Insertable, Queryable)]
pub struct Item {
    id: Uuid,
    name: String,
    price: f64,
    price_unit: String,
    producer_id: Uuid,
    description: String,
    post_date: String,
    tags: Vec<String>
}