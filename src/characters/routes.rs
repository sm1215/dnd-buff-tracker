use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use super::model::Character;

#[get("/characters")]
async fn find_all() -> impl Responder {
  HttpResponse::Ok().json(vec![
    Character {
      id: 1,
      name: String::from("Osel Mon"),
    },
    Character {
      id: 2,
      name: String::from("Terrance"),
    }
  ])
}

#[get("/characters/{id}")]
async fn find() -> impl Responder {
  HttpResponse::Ok().json(User {
    Character {
      id: 1,
      name: String::from("Osel Mon"),
    },
  })
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(find_all);
  config.service(find);
}