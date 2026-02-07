use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn subscribe() -> impl Responder {
    println!("Subsribed");
    HttpResponse::Ok().finish()
}

pub fn run(address: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(address)?
    .run();

    Ok(server)
}
