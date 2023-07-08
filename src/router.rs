use actix_web::{web, HttpResponse, Responder};
use crate::handler::{handle_get, handle_post};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/data")
            .route(web::get().to(handle_get))
            .route(web::post().to(handle_post)),
    );
}