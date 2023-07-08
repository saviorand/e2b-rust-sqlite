```rust
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::controllers::recipe_controller;

#[derive(Deserialize)]
pub struct RecipeForm {
    preferences: String,
}

#[get("/recipe")]
async fn get_recipe() -> impl Responder {
    let recipe = recipe_controller::get_recipe().await;
    HttpResponse::Ok().json(recipe)
}

#[post("/recipe")]
async fn create_recipe(form: web::Json<RecipeForm>) -> impl Responder {
    let recipe = recipe_controller::create_recipe(form.preferences.clone()).await;
    HttpResponse::Ok().json(recipe)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_recipe);
    cfg.service(create_recipe);
}
```