#[macro_use]
extern crate diesel;
use actix_web::{get, middleware, post, web, App, Error, HttpServer, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use listenfd::ListenFd;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Finds character by uid
#[get("/character/{character_id}")]
async fn get_character(
    pool: web::Data<DbPool>,
    character_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let character_uid = character_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let character = web::block(move || actions::find_character_by_uid(character_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(character) = character {
        Ok(HttpResponse::Ok().json(character))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", character_uid));
        Ok(res)
    }
}

// Inserts new character with name defined in form
#[post("/character")]
async fn add_character(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewCharacter>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let character = web::block(move || actions::insert_new_character(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(character))
}

// For rendering to templates
async fn render_tmpl(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

// For rendering to templates
struct AppData {
    tmpl: Tera
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:7000";

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        // templating engine
        // add html files here so they will be compiled
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
        ).unwrap();
        
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_character)
            .service(add_character)

            // method for binding templates for compiliation
            // .data(AppData {tmpl: tera})
    });

    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        println!("Server listening at: {:?}", listener);
        server.listen(listener)?
    } else {
        println!("Starting server at {}", &bind);
        server.bind(&bind)?
    };
    server.run().await
}
