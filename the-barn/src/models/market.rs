use uuid::UUID;
use crate address::Address;
use crate geolocation::Geolocation;

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