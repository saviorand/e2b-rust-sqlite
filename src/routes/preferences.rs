use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::controllers::preferences_controller;

#[derive(Deserialize)]
pub struct Preferences {
    pub flavor: String,
    pub sweetness: String,
    pub toppings: Vec<String>,
}

pub async fn get_preferences() -> impl Responder {
    let preferences = preferences_controller::get_preferences().await;
    HttpResponse::Ok().json(preferences)
}

pub async fn set_preferences(preferences: web::Json<Preferences>) -> impl Responder {
    let result = preferences_controller::set_preferences(preferences.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Preferences updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating preferences"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/preferences")
            .route(web::get().to(get_preferences))
            .route(web::post().to(set_preferences)),
    );
}