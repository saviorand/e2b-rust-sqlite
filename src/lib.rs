```rust
#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod models;
mod routes;
mod schema;
mod utils;

use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::Rocket;

pub fn rocket() -> Rocket {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .attach(AdHoc::on_attach("Database Migrations", move |rocket| {
            let conn = db::establish_connection(&database_url);
            match db::run_migrations(&conn) {
                Ok(()) => Ok(rocket),
                Err(e) => {
                    println!("Failed to run database migrations: {:?}", e);
                    Err(rocket)
                }
            }
        }))
        .mount("/", routes::routes())
}
```