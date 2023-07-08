```rust
use crate::models::preferences::Preferences;
use crate::services::preferences_service::PreferencesService;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_preferences() -> impl Responder {
    let preferences = PreferencesService::get_preferences().await;
    match preferences {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_preferences(preferences: web::Json<Preferences>) -> impl Responder {
    let result = PreferencesService::update_preferences(preferences.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
```