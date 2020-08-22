use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Session {
  pub id: i32,
  pub name: String,
  pub created: String,
}
