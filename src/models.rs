use serde::{Deserialize, Serialize};

use crate::schema::characters;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Character {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCharacter {
  pub name: String,
}
