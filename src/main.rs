mod controller;
mod property;
mod service;
mod restclient;
use controller::{get_controller, post_controller};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_controller::hello)
            .service(post_controller::echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
