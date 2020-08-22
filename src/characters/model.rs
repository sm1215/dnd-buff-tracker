use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Character {
  pub id: i32,
  pub name: String,
}
