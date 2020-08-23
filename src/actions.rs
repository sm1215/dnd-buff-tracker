use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

// Run query using Diesel to find existing row and return result
pub fn find_character_by_uid(
  uid: Uuid,
  conn: &SqliteConnection,
) -> Result<Option<models::Character>, diesel::result::Error> {
  use crate::schema::characters::dsl::*;

  let character = characters
    .filter(id.eq(uid.to_string()))
    .first::<models::Character>(conn)
    .optional()?;

  Ok(character)
}

// Run qury using Diesel to insert a new db row and return result
pub fn insert_new_character(
  // prevent collision with `name` column imported inside the function
  nm: &str,
  conn: &SqliteConnection,
) -> Result<models::User, diesel::result::Error> {
  // It is common when using Diesl with Actix to import schema-related
  // modules inside a function's scope (rather than the normal module's scope)
  // to prevent import collisions and namespace pollution.actions
  use crate::schema::characters::dsl::*;

  let new_character = models::Character {
    id: Uuid::new_v4().to_string(),
    name: nm.to_owned(),
  };

  diesel::insert_into(characters).values(&new_character).execute(conn)?;

  Ok(new_character)
}