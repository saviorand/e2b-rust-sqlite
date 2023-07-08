pub mod routes;
pub mod models;
pub mod controllers;
pub mod services;
pub mod utils;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

extern crate serde;

pub use self::routes::*;
pub use self::models::*;
pub use self::controllers::*;
pub use self::services::*;
pub use self::utils::*;