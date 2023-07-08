use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod router;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(router::router)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}