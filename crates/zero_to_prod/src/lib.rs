#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn greet(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(_request: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
