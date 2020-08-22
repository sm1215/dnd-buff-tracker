use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use listenfd::ListenFd;
// mod sessions;

async fn render_tmpl(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

struct AppData {
    tmpl: Tera
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // for autoreload
    // use: systemfd --no-pid -s http::7000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();
    // what is || doing and what else could we do with it? just an empty closure?
    let mut server = HttpServer::new(|| {
        // templating engine
        // add html files here so they will be compiled
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
        ).unwrap();
        
        App::new()
            .data(AppData {tmpl: tera})
            // .configure(users::init_routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:7000")?
    };
    server.run().await

}
