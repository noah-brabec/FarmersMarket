use uuid::UUID;
use super::address::Address;
use super::geolocation::Geolocation;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct market {
    id: UUID,
    name: String,
    address: Address,
    description: String,
    email: String,
    phone: String,
    website: String
}