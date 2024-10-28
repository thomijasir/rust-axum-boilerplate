use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: u64,
  pub name: String,
  pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
  pub name: String,
  pub email: String,
}
