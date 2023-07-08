use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Milkshake Recipe Generator!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}