```rust
#[macro_use]
extern crate rocket;

mod routes;
mod models;
mod controllers;
mod services;
mod utils;

use routes::{home, preferences, recipe};

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![home::index, preferences::get, preferences::post, recipe::get])
        .attach(utils::db_connector::stage())
}
```