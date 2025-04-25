use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub user_id: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
}
