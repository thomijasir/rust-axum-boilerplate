use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub user_id: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
}
