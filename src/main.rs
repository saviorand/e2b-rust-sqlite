```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod routes;
mod models;
mod controllers;
mod services;
mod utils;

use routes::{home, preferences, recipe};
use rocket_contrib::serve::StaticFiles;
use utils::db_connector::DbConn;

fn main() {
    rocket::ignite()
        .mount("/", routes![home::index, preferences::get, preferences::post, recipe::get])
        .mount("/public", StaticFiles::from("static"))
        .attach(DbConn::fairing())
        .launch();
}
```