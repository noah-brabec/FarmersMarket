use uuid::Uuid;

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