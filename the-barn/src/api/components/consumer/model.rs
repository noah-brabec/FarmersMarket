use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct Consumer {
    id: Uuid,
    name: String,
    address: serde_json::Value,
    type_: String,
    description: Option<String>,
    pub markets: Option<Vec<Uuid>>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>
}