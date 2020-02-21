extern crate serde;
extern crate serde_json;

use uuid::Uuid;
use crate::geolocation::Geolocation;
use crate::address::Address;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct UserGroup {
    group_id: UUID,
    user_id: UUID,
    organization_id: UUID,
    role: String
}