extern crate serde;
extern crate serde_json;

use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct UserGroup {
    group_id: Uuid,
    user_id: Uuid,
    organization_id: Uuid,
    role: String
}