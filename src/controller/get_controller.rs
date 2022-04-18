use actix_web::{get, HttpResponse, Responder};
use crate::service::main_service;

#[get("/")]
pub async fn hello() -> impl Responder {

    main_service::hoge().await;

    HttpResponse::Ok().body("Hello world!")
}
