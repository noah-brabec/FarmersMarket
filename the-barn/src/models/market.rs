use uuid::UUID;
use super::address::Address;
use super::geolocation::Geolocation;

pub struct market {
    id: UUID,
    name: String,
    address: Address,
    geolocation: Geolocation,
    description: String,
    email: String,
    phone: String,
    website: String
}