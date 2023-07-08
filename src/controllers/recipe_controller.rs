```rust
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::services::recipe_service;
use crate::models::recipe::Recipe;

pub async fn get_recipe(data: web::Data<Recipe>) -> impl Responder {
    match recipe_service::get_recipe(data.0).await {
        Ok(recipe) => HttpResponse::Ok().json(recipe),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Error while fetching recipe"})),
    }
}

pub async fn create_recipe(data: web::Json<Recipe>) -> impl Responder {
    match recipe_service::create_recipe(data.into_inner()).await {
        Ok(recipe) => HttpResponse::Created().json(recipe),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Error while creating recipe"})),
    }
}

pub async fn update_recipe(data: web::Json<Recipe>) -> impl Responder {
    match recipe_service::update_recipe(data.into_inner()).await {
        Ok(recipe) => HttpResponse::Ok().json(recipe),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Error while updating recipe"})),
    }
}

pub async fn delete_recipe(data: web::Data<Recipe>) -> impl Responder {
    match recipe_service::delete_recipe(data.0).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Error while deleting recipe"})),
    }
}
```