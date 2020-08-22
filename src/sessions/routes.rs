use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use super::model::Session;

#[get("/sessions")]
async fn find_all() -> impl Responder {
  HttpResponse::Ok().json(vec![
    Session {
      id: 1,
      name: "In Town",
      created: "8-22-2020"
    }
  ])
}