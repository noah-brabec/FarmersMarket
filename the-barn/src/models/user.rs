use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct User {
    id: Uuid,
    first: String,
    last: String,
    email: String,
    phone: Option<String>,
    password: String
}