use uuid::UUID;

pub struct Item {
    id: UUID,
    name: String,
    price: f64,
    price_unit: String,
    producer_id: UUID,
    description: String,
    post_date: String,
    tags: Vec<String>
}