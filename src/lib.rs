use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use std::net::TcpListener;
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FromData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct FromData {
    email: String,
    name: String,
}


pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    }).listen(listener)?
        .run();
    Ok(server)
}
