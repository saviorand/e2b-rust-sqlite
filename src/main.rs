```rust
use actix_web::{web, App, HttpServer};
use std::io;

mod routes;
mod controllers;
mod services;
mod models;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .configure(routes::home::config)
                    .configure(routes::preferences::config)
                    .configure(routes::recipe::config)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```