```rust
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::DataModel;
use crate::db;

#[derive(Deserialize)]
pub struct Info {
    data: String,
}

pub async fn handle_post(info: web::Json<Info>) -> impl Responder {
    let connection = db::establish_connection();
    let new_data = DataModel {
        data: info.data.clone(),
    };
    db::create_data(&connection, new_data);
    HttpResponse::Ok().body("Data saved successfully")
}

pub async fn handle_get() -> impl Responder {
    let connection = db::establish_connection();
    let results = db::query_data(&connection);
    HttpResponse::Ok().json(results)
}
```